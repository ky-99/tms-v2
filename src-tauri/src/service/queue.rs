use chrono::Utc;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::error::ServiceError;
use crate::models::queue::{NewQueueEntry, QueueEntry, QueueEntryWithTask};
use crate::models::task::TaskStatus;
use crate::schema::{task_queue, tasks};
use crate::service::task::TaskService;

/// QueueService: タスクキュー管理操作を提供
pub struct QueueService;

impl QueueService {
    /// キュー全体を取得（タスク情報含む）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    ///
    /// # Returns
    /// * `Ok(Vec<QueueEntryWithTask>)` - キューエントリリスト（position順）
    /// * `Err(ServiceError)` - エラー
    pub fn get_queue(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<QueueEntryWithTask>, ServiceError> {
        let results = task_queue::table
            .inner_join(tasks::table.on(task_queue::task_id.eq(tasks::id)))
            .select((
                task_queue::task_id,
                task_queue::position,
                task_queue::added_at,
                tasks::title,
                tasks::status,
                tasks::description,
            ))
            .order(task_queue::position.asc())
            .load::<(String, i32, String, String, String, Option<String>)>(conn)?;

        let queue_entries = results
            .into_iter()
            .map(
                |(task_id, position, added_at, task_title, task_status, task_description)| {
                    QueueEntryWithTask {
                        task_id,
                        position,
                        added_at,
                        task_title,
                        task_status,
                        task_description,
                    }
                },
            )
            .collect();

        Ok(queue_entries)
    }

    /// タスクをキューに追加
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    ///
    /// # Returns
    /// * `Ok(QueueEntry)` - 追加されたキューエントリ
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Business Logic
    /// - タスクが存在すること
    /// - タスクがキューに既に存在しないこと
    /// - **キューへ追加時、タスクのステータスを自動的にActiveに変更**
    pub fn add_to_queue(
        conn: &mut SqliteConnection,
        task_id: String,
    ) -> Result<QueueEntry, ServiceError> {
        // タスクが存在するか確認
        let task = tasks::table
            .find(&task_id)
            .first::<crate::models::task::Task>(conn)
            .optional()?;

        task.ok_or_else(|| ServiceError::TaskNotFound(task_id.clone()))?;

        // 既にキューに存在するか確認
        let existing = task_queue::table
            .find(&task_id)
            .first::<QueueEntry>(conn)
            .optional()?;

        if existing.is_some() {
            return Err(ServiceError::DuplicateQueueEntry(task_id));
        }

        // 【新規追加】子タスクを持つ親タスクはキューに追加できない（BR-015）
        if TaskService::has_children(conn, &task_id)? {
            return Err(ServiceError::TaskHasChildren(task_id));
        }

        // トランザクション内で処理
        conn.transaction::<QueueEntry, ServiceError, _>(|conn| {
            // タスクのステータスをActiveに更新
            diesel::update(tasks::table.find(&task_id))
                .set(tasks::status.eq(TaskStatus::Active.as_str()))
                .execute(conn)?;

            // 【新規追加】親ステータス更新
            TaskService::update_parent_status_if_needed(conn, &task_id)?;

            // 現在の最大positionを取得
            let max_position: Option<i32> = task_queue::table
                .select(diesel::dsl::max(task_queue::position))
                .first(conn)?;

            let new_position = max_position.unwrap_or(-1) + 1;

            // 新規エントリ作成
            let new_entry = NewQueueEntry::new(task_id.clone(), new_position);

            // キューに挿入
            diesel::insert_into(task_queue::table)
                .values(&new_entry)
                .execute(conn)?;

            // 挿入されたエントリを取得
            let entry = task_queue::table.find(&task_id).first::<QueueEntry>(conn)?;

            Ok(entry)
        })
    }

    /// タスクをキューから削除
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    ///
    /// # Returns
    /// * `Ok(())` - 削除成功
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Business Logic
    /// - **キューから削除時、タスクのステータスを自動更新**
    ///   - 現在のステータスがDraft → Archivedに変更
    ///   - 現在のステータスがCompleted → Completedのまま（変更なし）
    ///   - それ以外（Active等） → Draftに変更
    pub fn remove_from_queue(
        conn: &mut SqliteConnection,
        task_id: String,
        target_status: String,
    ) -> Result<(), ServiceError> {
        // target_statusのバリデーション
        if target_status != "draft" && target_status != "completed" {
            return Err(ServiceError::InvalidTaskStatus(target_status));
        }

        // エントリが存在するか確認
        let entry = task_queue::table
            .find(&task_id)
            .first::<QueueEntry>(conn)
            .optional()?;

        if entry.is_none() {
            return Err(ServiceError::QueueEntryNotFound(task_id.clone()));
        }

        let removed_position = entry.unwrap().position;

        // トランザクション内で処理
        conn.transaction::<(), ServiceError, _>(|conn| {
            // エントリ削除
            diesel::delete(task_queue::table.find(&task_id)).execute(conn)?;

            // 削除されたエントリより後ろのpositionを1つずつ繰り上げ
            diesel::update(task_queue::table.filter(task_queue::position.gt(removed_position)))
                .set(task_queue::position.eq(task_queue::position - 1))
                .execute(conn)?;

            // 指定されたステータスに更新（updated_atも同時更新）
            let now = Utc::now().to_rfc3339();
            diesel::update(tasks::table.find(&task_id))
                .set((
                    tasks::status.eq(&target_status),
                    tasks::updated_at.eq(&now),
                ))
                .execute(conn)?;

            // 【新規追加】親ステータス更新
            TaskService::update_parent_status_if_needed(conn, &task_id)?;

            Ok(())
        })
    }

    /// キュー内の全タスクを完了状態にする
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    ///
    /// # Returns
    /// * `Ok(usize)` - 完了したタスク数
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Business Logic
    /// - **全タスクを完了状態に更新**
    ///   - 全タスクのステータスを"completed"に変更
    ///   - 全タスクのupdated_atを現在時刻に更新
    ///   - 親ステータスを更新（子タスクの場合）
    ///   - キュー全体を削除
    ///   - トランザクション内で実行（all or nothing）
    pub fn complete_all_queue(conn: &mut SqliteConnection) -> Result<usize, ServiceError> {
        // キュー内の全タスクIDを取得
        let task_ids: Vec<String> = task_queue::table
            .select(task_queue::task_id)
            .load::<String>(conn)?;

        let completed_count = task_ids.len();

        // トランザクション内で処理
        conn.transaction::<(), ServiceError, _>(|conn| {
            let now = Utc::now().to_rfc3339();

            // 各タスクをcompletedステータスに更新
            for task_id in &task_ids {
                diesel::update(tasks::table.find(task_id))
                    .set((
                        tasks::status.eq(TaskStatus::Completed.as_str()),
                        tasks::updated_at.eq(&now),
                    ))
                    .execute(conn)?;
            }

            // キュー全体を削除
            diesel::delete(task_queue::table).execute(conn)?;

            // 親ステータスを更新
            for task_id in &task_ids {
                TaskService::update_parent_status_if_needed(conn, task_id)?;
            }

            Ok(())
        })?;

        Ok(completed_count)
    }

    /// キュー全体をクリア
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    ///
    /// # Returns
    /// * `Ok(())` - クリア成功
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Business Logic
    /// - **全タスクのステータスを更新**
    ///   - Draft → Archived
    ///   - Completed → Completed（変更なし）
    ///   - それ以外 → Draft
    pub fn clear_queue(conn: &mut SqliteConnection) -> Result<(), ServiceError> {
        // キュー内の全タスクIDを取得
        let task_ids: Vec<String> = task_queue::table
            .select(task_queue::task_id)
            .load::<String>(conn)?;

        // トランザクション内で処理
        conn.transaction::<(), ServiceError, _>(|conn| {
            // 各タスクのステータスを更新
            for task_id in &task_ids {
                let current_task = tasks::table
                    .find(task_id)
                    .first::<crate::models::task::Task>(conn)?;

                let new_status = match current_task.status.as_str() {
                    "draft" => TaskStatus::Archived.as_str(),
                    "completed" => TaskStatus::Completed.as_str(),
                    _ => TaskStatus::Draft.as_str(),
                };

                diesel::update(tasks::table.find(task_id))
                    .set(tasks::status.eq(new_status))
                    .execute(conn)?;
            }

            // キュー全体を削除
            diesel::delete(task_queue::table).execute(conn)?;

            // 【新規追加】親ステータスを更新（重複は update_parent_status_if_needed 内で処理される）
            for task_id in &task_ids {
                TaskService::update_parent_status_if_needed(conn, task_id)?;
            }

            Ok(())
        })
    }

    /// タスクのキュー内位置を更新
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    /// * `new_position` - 新しい位置（0始まり）
    ///
    /// # Returns
    /// * `Ok(QueueEntry)` - 更新されたキューエントリ
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Note
    /// 他のエントリの位置も自動的に調整されます
    /// ステータスは変更されません
    pub fn update_queue_position(
        conn: &mut SqliteConnection,
        task_id: String,
        new_position: i32,
    ) -> Result<QueueEntry, ServiceError> {
        // エントリが存在するか確認
        let entry = task_queue::table
            .find(&task_id)
            .first::<QueueEntry>(conn)
            .optional()?;

        let entry = entry.ok_or_else(|| ServiceError::QueueEntryNotFound(task_id.clone()))?;

        let old_position = entry.position;

        // 新しい位置が範囲内か確認
        let queue_size: i64 = task_queue::table.count().get_result(conn)?;
        if new_position < 0 || new_position >= queue_size as i32 {
            return Err(ServiceError::InvalidInput(format!(
                "Invalid position: {}. Queue size: {}",
                new_position, queue_size
            )));
        }

        // 位置が変わらない場合は何もしない
        if old_position == new_position {
            return Ok(entry);
        }

        // トランザクション内で位置を更新
        conn.transaction::<_, ServiceError, _>(|conn| {
            if old_position < new_position {
                // 下に移動: old_position < pos <= new_position の範囲を1つ上にシフト
                diesel::update(
                    task_queue::table.filter(
                        task_queue::position
                            .gt(old_position)
                            .and(task_queue::position.le(new_position)),
                    ),
                )
                .set(task_queue::position.eq(task_queue::position - 1))
                .execute(conn)?;
            } else {
                // 上に移動: new_position <= pos < old_position の範囲を1つ下にシフト
                diesel::update(
                    task_queue::table.filter(
                        task_queue::position
                            .ge(new_position)
                            .and(task_queue::position.lt(old_position)),
                    ),
                )
                .set(task_queue::position.eq(task_queue::position + 1))
                .execute(conn)?;
            }

            // 対象タスクの位置を更新
            diesel::update(task_queue::table.find(&task_id))
                .set(task_queue::position.eq(new_position))
                .execute(conn)?;

            Ok(())
        })?;

        // 更新されたエントリを取得
        let updated_entry = task_queue::table
            .find(&task_id)
            .first::<QueueEntry>(conn)?;

        Ok(updated_entry)
    }

    /// キュー全体を一括で並び替え
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_ids` - 新しい順序でのタスクIDリスト
    ///
    /// # Returns
    /// * `Ok(Vec<QueueEntry>)` - 並び替え後のキューエントリリスト
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Validation
    /// - 全タスクIDがキューに存在すること
    /// - 現在のキューサイズとタスクIDリストのサイズが一致すること
    ///
    /// # Note
    /// ステータスは変更されません
    pub fn reorder_queue(
        conn: &mut SqliteConnection,
        task_ids: Vec<String>,
    ) -> Result<Vec<QueueEntry>, ServiceError> {
        // 現在のキューサイズを取得
        let current_queue_size: i64 = task_queue::table.count().get_result(conn)?;

        // タスクIDリストのサイズが一致するか確認
        if task_ids.len() != current_queue_size as usize {
            return Err(ServiceError::InvalidInput(format!(
                "Task ID list size ({}) does not match queue size ({})",
                task_ids.len(),
                current_queue_size
            )));
        }

        // 全タスクIDがキューに存在するか確認
        for task_id in &task_ids {
            let exists = task_queue::table
                .find(task_id)
                .first::<QueueEntry>(conn)
                .optional()?;

            if exists.is_none() {
                return Err(ServiceError::QueueEntryNotFound(task_id.clone()));
            }
        }

        // トランザクション内で一括更新
        conn.transaction::<_, ServiceError, _>(|conn| {
            for (index, task_id) in task_ids.iter().enumerate() {
                diesel::update(task_queue::table.find(task_id))
                    .set(task_queue::position.eq(index as i32))
                    .execute(conn)?;
            }
            Ok(())
        })?;

        // 並び替え後のキューを取得
        Self::get_queue_entries(conn)
    }

    /// キューエントリのみを取得（内部用ヘルパー）
    fn get_queue_entries(conn: &mut SqliteConnection) -> Result<Vec<QueueEntry>, ServiceError> {
        let entries = task_queue::table
            .order(task_queue::position.asc())
            .load::<QueueEntry>(conn)?;
        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::CreateTaskRequest;
    use crate::service::TaskService;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    fn setup_test_db() -> SqliteConnection {
        let mut conn =
            SqliteConnection::establish(":memory:").expect("Failed to create in-memory database");

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");

        conn
    }

    #[test]
    fn test_add_to_queue_success() {
        let mut conn = setup_test_db();

        // テストタスク作成
        let req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, req).unwrap();

        // キューに追加
        let result = QueueService::add_to_queue(&mut conn, task.id.clone());
        assert!(result.is_ok());

        let entry = result.unwrap();
        assert_eq!(entry.task_id, task.id);
        assert_eq!(entry.position, 0);

        // タスクのステータスがActiveになっているか確認
        let updated_task = TaskService::get_task(&mut conn, &task.id).unwrap();
        assert_eq!(updated_task.status, TaskStatus::Active);
    }

    #[test]
    fn test_add_to_queue_task_not_found() {
        let mut conn = setup_test_db();

        let result = QueueService::add_to_queue(&mut conn, "non-existent-id".to_string());
        assert!(result.is_err());

        if let Err(ServiceError::TaskNotFound(id)) = result {
            assert_eq!(id, "non-existent-id");
        } else {
            panic!("Expected TaskNotFound error");
        }
    }

    #[test]
    fn test_add_to_queue_duplicate() {
        let mut conn = setup_test_db();

        // テストタスク作成とキューに追加
        let req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, req).unwrap();
        QueueService::add_to_queue(&mut conn, task.id.clone()).unwrap();

        // 同じタスクを再度追加しようとする
        let result = QueueService::add_to_queue(&mut conn, task.id.clone());
        assert!(result.is_err());

        if let Err(ServiceError::DuplicateQueueEntry(_)) = result {
            // OK
        } else {
            panic!("Expected DuplicateQueueEntry error");
        }
    }

    #[test]
    fn test_get_queue() {
        let mut conn = setup_test_db();

        // 複数のタスクを作成してキューに追加
        for i in 1..=3 {
            let req = CreateTaskRequest {
                title: format!("Task {}", i),
                description: None,
                tags: vec![],
                parent_id: None,
            };
            let task = TaskService::create_task(&mut conn, req).unwrap();
            QueueService::add_to_queue(&mut conn, task.id).unwrap();
        }

        // キュー取得
        let result = QueueService::get_queue(&mut conn);
        assert!(result.is_ok());

        let queue = result.unwrap();
        assert_eq!(queue.len(), 3);
        assert_eq!(queue[0].task_title, "Task 1");
        assert_eq!(queue[1].task_title, "Task 2");
        assert_eq!(queue[2].task_title, "Task 3");
    }

    #[test]
    fn test_remove_from_queue_success() {
        let mut conn = setup_test_db();

        // タスク作成とキューに追加
        let req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, req).unwrap();
        QueueService::add_to_queue(&mut conn, task.id.clone()).unwrap();

        // キューから削除（draftに戻す）
        let result = QueueService::remove_from_queue(&mut conn, task.id.clone(), "draft".to_string());
        assert!(result.is_ok());

        // タスクのステータスがDraftに戻っているか確認
        let updated_task = TaskService::get_task(&mut conn, &task.id).unwrap();
        assert_eq!(updated_task.status, TaskStatus::Draft);
    }

    #[test]
    fn test_remove_from_queue_not_found() {
        let mut conn = setup_test_db();

        let result = QueueService::remove_from_queue(&mut conn, "non-existent-id".to_string(), "draft".to_string());
        assert!(result.is_err());

        if let Err(ServiceError::QueueEntryNotFound(_)) = result {
            // OK
        } else {
            panic!("Expected QueueEntryNotFound error");
        }
    }

    #[test]
    fn test_clear_queue() {
        let mut conn = setup_test_db();

        // 複数のタスクを作成してキューに追加
        for i in 1..=3 {
            let req = CreateTaskRequest {
                title: format!("Task {}", i),
                description: None,
                tags: vec![],
                parent_id: None,
            };
            let task = TaskService::create_task(&mut conn, req).unwrap();
            QueueService::add_to_queue(&mut conn, task.id).unwrap();
        }

        // キュー全体をクリア
        let result = QueueService::clear_queue(&mut conn);
        assert!(result.is_ok());

        // キューが空になっているか確認
        let queue = QueueService::get_queue(&mut conn).unwrap();
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_update_queue_position() {
        let mut conn = setup_test_db();

        // 3つのタスクを作成してキューに追加
        let mut task_ids = Vec::new();
        for i in 1..=3 {
            let req = CreateTaskRequest {
                title: format!("Task {}", i),
                description: None,
                tags: vec![],
                parent_id: None,
            };
            let task = TaskService::create_task(&mut conn, req).unwrap();
            QueueService::add_to_queue(&mut conn, task.id.clone()).unwrap();
            task_ids.push(task.id);
        }

        // 最初のタスク（position=0）を最後（position=2）に移動
        let result =
            QueueService::update_queue_position(&mut conn, task_ids[0].clone(), 2);
        assert!(result.is_ok());

        // キューを確認
        let queue = QueueService::get_queue(&mut conn).unwrap();
        assert_eq!(queue[0].task_title, "Task 2");
        assert_eq!(queue[1].task_title, "Task 3");
        assert_eq!(queue[2].task_title, "Task 1");
    }

    #[test]
    fn test_reorder_queue() {
        let mut conn = setup_test_db();

        // 3つのタスクを作成してキューに追加
        let mut task_ids = Vec::new();
        for i in 1..=3 {
            let req = CreateTaskRequest {
                title: format!("Task {}", i),
                description: None,
                tags: vec![],
                parent_id: None,
            };
            let task = TaskService::create_task(&mut conn, req).unwrap();
            QueueService::add_to_queue(&mut conn, task.id.clone()).unwrap();
            task_ids.push(task.id);
        }

        // 順序を逆にする
        let new_order = vec![task_ids[2].clone(), task_ids[1].clone(), task_ids[0].clone()];
        let result = QueueService::reorder_queue(&mut conn, new_order);
        assert!(result.is_ok());

        // キューを確認
        let queue = QueueService::get_queue(&mut conn).unwrap();
        assert_eq!(queue[0].task_title, "Task 3");
        assert_eq!(queue[1].task_title, "Task 2");
        assert_eq!(queue[2].task_title, "Task 1");
    }

    #[test]
    fn test_add_to_queue_parent_task_rejected() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "親タスク".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // 子タスク作成
        TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "子タスク".to_string(),
                description: None,
                tags: vec![],
                parent_id: Some(parent.id.clone()),
            },
        )
        .unwrap();

        // 親タスクをキューに追加しようとする（失敗するはず）
        let result = QueueService::add_to_queue(&mut conn, parent.id.clone());
        assert!(result.is_err());

        // エラーの種類を確認
        match result.unwrap_err() {
            ServiceError::TaskHasChildren(_) => {
                // 期待通りのエラー
            }
            _ => panic!("Expected TaskHasChildren error"),
        }
    }

    #[test]
    fn test_add_to_queue_child_task_success() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "親タスク".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // 子タスク作成
        let child = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "子タスク".to_string(),
                description: None,
                tags: vec![],
                parent_id: Some(parent.id.clone()),
            },
        )
        .unwrap();

        // 子タスクをキューに追加する（成功するはず）
        let result = QueueService::add_to_queue(&mut conn, child.id.clone());
        assert!(result.is_ok());

        // キューを確認
        let queue = QueueService::get_queue(&mut conn).unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].task_title, "子タスク");
    }

    #[test]
    fn test_remove_from_queue_updates_updated_at() {
        let mut conn = setup_test_db();

        // タスク作成
        let task = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Test Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // キューに追加
        QueueService::add_to_queue(&mut conn, task.id.clone()).unwrap();

        // 元のupdated_atを記録
        let original_updated_at = task.updated_at.clone();

        // 少し待機（タイムスタンプが確実に変わるように）
        std::thread::sleep(std::time::Duration::from_millis(10));

        // キューから削除（completedに変更）
        QueueService::remove_from_queue(&mut conn, task.id.clone(), "completed".to_string())
            .unwrap();

        // タスクを再取得してupdated_atを確認
        let updated_task = TaskService::get_task(&mut conn, &task.id).unwrap();

        // updated_atが更新されていることを確認
        assert_ne!(
            updated_task.updated_at, original_updated_at,
            "updated_at should be updated when task status changes to completed"
        );

        // ステータスがcompletedに変更されていることを確認
        assert_eq!(updated_task.status, TaskStatus::Completed);
    }

    #[test]
    fn test_can_queue_parent_with_only_archived_children() {
        let mut conn = setup_test_db();

        // 親タスク作成（Draft）
        let parent = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Parent Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // 子タスク作成（Draft）
        let child = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Child Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: Some(parent.id.clone()),
            },
        )
        .unwrap();

        // 子タスクが存在するので親タスクをキューに追加できない
        let result = QueueService::add_to_queue(&mut conn, parent.id.clone());
        assert!(
            result.is_err(),
            "子タスクが存在する場合、親タスクはキューに追加できないべき"
        );
        assert!(
            matches!(result.unwrap_err(), ServiceError::TaskHasChildren(_)),
            "TaskHasChildrenエラーが返されるべき"
        );

        // 子タスクをアーカイブ
        TaskService::delete_task(&mut conn, &child.id).unwrap();

        // 子タスクが全てアーカイブされたので親タスクをキューに追加できる
        let result = QueueService::add_to_queue(&mut conn, parent.id.clone());
        assert!(
            result.is_ok(),
            "全ての子タスクがアーカイブされた場合、親タスクはキューに追加できるべき"
        );

        // キューに追加されていることを確認
        let queue_entry = result.unwrap();
        assert_eq!(queue_entry.task_id, parent.id);
    }

    #[test]
    fn test_cannot_queue_parent_with_active_children() {
        let mut conn = setup_test_db();

        // 親タスク作成（Draft）
        let parent = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Parent Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // 子タスク1作成（Draft）
        let child1 = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Child 1".to_string(),
                description: None,
                tags: vec![],
                parent_id: Some(parent.id.clone()),
            },
        )
        .unwrap();

        // 子タスク2作成（Draft）
        let child2 = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Child 2".to_string(),
                description: None,
                tags: vec![],
                parent_id: Some(parent.id.clone()),
            },
        )
        .unwrap();

        // 子タスク1をアーカイブ
        TaskService::delete_task(&mut conn, &child1.id).unwrap();

        // まだ1つのDraft子タスクが残っているので親タスクをキューに追加できない
        let result = QueueService::add_to_queue(&mut conn, parent.id.clone());
        assert!(
            result.is_err(),
            "1つでもアクティブな子タスクが存在する場合、親タスクはキューに追加できないべき"
        );
        assert!(
            matches!(result.unwrap_err(), ServiceError::TaskHasChildren(_)),
            "TaskHasChildrenエラーが返されるべき"
        );

        // 子タスク2もアーカイブ
        TaskService::delete_task(&mut conn, &child2.id).unwrap();

        // 全ての子タスクがアーカイブされたので親タスクをキューに追加できる
        let result = QueueService::add_to_queue(&mut conn, parent.id.clone());
        assert!(
            result.is_ok(),
            "全ての子タスクがアーカイブされた場合、親タスクはキューに追加できるべき"
        );
    }
}
