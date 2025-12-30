use chrono::Utc;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::error::ServiceError;
use crate::models::task::{
    CreateTaskRequest, ListTasksPaginatedParams, NewTask, PaginatedTaskResponse,
    SearchTasksParams, Task, TaskHierarchyResponse, TaskResponse, TaskStatus, UpdateTaskRequest,
    UpdateTaskRequestInput,
};
use crate::schema::{task_tags, tags, tasks};

/// TaskService: タスクCRUD操作を提供
pub struct TaskService;

impl TaskService {
    /// 新規タスクを作成
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `req` - タスク作成リクエスト
    ///
    /// # Returns
    /// * `Ok(TaskResponse)` - 作成されたタスク（タグ含む）
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Validation
    /// - タイトルが空でないこと
    /// - 親タスクが指定されている場合、存在すること
    pub fn create_task(
        conn: &mut SqliteConnection,
        req: CreateTaskRequest,
    ) -> Result<TaskResponse, ServiceError> {
        // バリデーション: タイトルが空でないこと
        if req.title.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "Title cannot be empty".to_string(),
            ));
        }

        // バリデーション: 親タスクが存在するか確認（指定されている場合）
        if let Some(ref parent_id) = req.parent_id {
            let parent_exists = tasks::table
                .find(parent_id)
                .select(tasks::id)
                .first::<String>(conn)
                .optional()?;

            if parent_exists.is_none() {
                return Err(ServiceError::ParentTaskNotFound(parent_id.clone()));
            }

            // 【新規追加】階層深度チェック（孫タスク作成禁止 - BR-016）
            Self::validate_hierarchy_depth(conn, parent_id)?;
        }

        // NewTask作成
        let new_task = NewTask::from_request(req.clone());
        let task_id = new_task.id.clone();

        // タスク挿入
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(conn)?;

        // タグの関連付け（tags配列が空でない場合）
        if !req.tags.is_empty() {
            // task_tags中間テーブルに挿入
            for tag_name in &req.tags {
                // タグが存在するか確認（簡易実装: 外部キー制約に任せる）
                // TODO: TagServiceと連携してタグのusage_countをインクリメント
                use crate::schema::tags;

                // タグIDを取得
                let tag_id_result = tags::table
                    .filter(tags::name.eq(tag_name))
                    .select(tags::id)
                    .first::<String>(conn)
                    .optional()?;

                if let Some(tag_id) = tag_id_result {
                    // task_tags挿入
                    diesel::insert_into(task_tags::table)
                        .values((
                            task_tags::task_id.eq(&task_id),
                            task_tags::tag_id.eq(&tag_id),
                        ))
                        .execute(conn)?;
                } else {
                    return Err(ServiceError::TagNotFound(tag_name.clone()));
                }
            }
        }

        // 作成されたタスクを取得
        let created_task = tasks::table.find(&task_id).first::<Task>(conn)?;

        // タグ名リストを取得
        let tag_names = if req.tags.is_empty() {
            Vec::new()
        } else {
            req.tags
        };

        Ok(created_task.with_tags(tag_names))
    }

    /// 全タスクを取得（簡易版 - Draft + Active）
    /// タスク一覧を取得（ステータスフィルタ対応）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `status` - ステータスフィルタ（複数指定可能）
    ///
    /// # Returns
    /// * `Ok(Vec<TaskResponse>)` - タスク一覧
    /// * `Err(ServiceError)` - データベースエラー
    ///
    /// # Validation
    /// - status = None: Draft + Active（デフォルト、後方互換性維持）
    /// - status = Some(vec!["completed"]): Completedのみ
    /// - status = Some(vec!["archived"]): Archivedのみ
    /// - status = Some(vec!["draft", "active"]): Draft + Active
    ///
    /// # Notes
    /// - タスクプール、CompletedPage、ArchivedPageで使用
    /// - REQ-0019: statusパラメータ対応
    pub fn list_tasks(
        conn: &mut SqliteConnection,
        status: Option<Vec<String>>,
    ) -> Result<Vec<TaskResponse>, ServiceError> {
        let mut query = tasks::table.into_boxed();

        // ステータスフィルタ
        if let Some(statuses) = status {
            if statuses.is_empty() {
                // 空配列の場合、空の結果を返す（フィルタ条件なし = マッチなし）
                return Ok(Vec::new());
            }
            query = query.filter(tasks::status.eq_any(statuses));
        } else {
            // デフォルト: Draft + Active（後方互換性維持）
            query = query.filter(tasks::status.eq("draft").or(tasks::status.eq("active")));
        }

        // タスク取得
        let tasks = query.order(tasks::created_at.desc()).load::<Task>(conn)?;

        // 各タスクにタグと子タスクIDを追加
        tasks
            .into_iter()
            .map(|task| Self::enrich_task_response(conn, task))
            .collect()
    }

    /// タスク一覧を取得（ページネーション対応）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `params` - ページネーションパラメータ（status, limit, offset）
    ///
    /// # Returns
    /// * `PaginatedTaskResponse` - タスクリストと総件数
    ///
    /// # Examples
    /// - デフォルト（limit=20, offset=0）で Draft + Active を取得
    /// - CompletedPage/ArchivedPageでページネーション表示
    /// - REQ-0024: ページネーション機能実装
    pub fn list_tasks_paginated(
        conn: &mut SqliteConnection,
        params: ListTasksPaginatedParams,
    ) -> Result<PaginatedTaskResponse, ServiceError> {
        // デフォルト値の設定
        let limit = params.limit.unwrap_or(20);
        let offset = params.offset.unwrap_or(0);

        // ステータスフィルタの条件を取得
        let status_filter = if let Some(statuses) = params.status {
            if statuses.is_empty() {
                // 空配列の場合、空の結果を返す
                return Ok(PaginatedTaskResponse {
                    tasks: Vec::new(),
                    total: 0,
                });
            }
            Some(statuses)
        } else {
            None
        };

        // 総件数取得用クエリ
        let mut count_query = tasks::table.into_boxed();
        if let Some(ref statuses) = status_filter {
            count_query = count_query.filter(tasks::status.eq_any(statuses));
        } else {
            // デフォルト: Draft + Active（後方互換性維持）
            count_query =
                count_query.filter(tasks::status.eq("draft").or(tasks::status.eq("active")));
        }
        let total = count_query.count().get_result::<i64>(conn)?;

        // タスク取得用クエリ
        let mut data_query = tasks::table.into_boxed();
        if let Some(statuses) = status_filter {
            data_query = data_query.filter(tasks::status.eq_any(statuses));
        } else {
            // デフォルト: Draft + Active（後方互換性維持）
            data_query = data_query.filter(tasks::status.eq("draft").or(tasks::status.eq("active")));
        }

        // タスク取得（limit/offset適用）
        let tasks = data_query
            .order(tasks::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Task>(conn)?;

        // 親タイトルのバッチ取得（パフォーマンス最適化）
        use std::collections::HashMap;
        let parent_ids: Vec<String> = tasks
            .iter()
            .filter_map(|t| t.parent_id.as_ref())
            .cloned()
            .collect();

        let parent_titles: HashMap<String, String> = if !parent_ids.is_empty() {
            tasks::table
                .filter(tasks::id.eq_any(parent_ids))
                .select((tasks::id, tasks::title))
                .load::<(String, String)>(conn)?
                .into_iter()
                .collect()
        } else {
            HashMap::new()
        };

        // 各タスクにタグ・子タスクID・親タイトルを追加
        let enriched_tasks: Result<Vec<TaskResponse>, ServiceError> = tasks
            .into_iter()
            .map(|task| {
                let parent_title = task
                    .parent_id
                    .as_ref()
                    .and_then(|pid| parent_titles.get(pid).cloned());
                Self::enrich_task_response_with_parent_title(conn, task, parent_title)
            })
            .collect();

        Ok(PaginatedTaskResponse {
            tasks: enriched_tasks?,
            total,
        })
    }

    /// タスク階層を取得（Draft/Active な親タスク + その子タスク）
    ///
    /// # Returns
    /// * Draft または Active なルートタスク（parentId なし）の配列
    /// * 各タスクは children フィールドに子タスクを含む
    /// * 子タスクは Draft、Active、Completed を含む（Archived は除外）
    ///
    /// # Note
    /// BR-016により階層は最大2層（親-子のみ、孫タスク禁止）
    pub fn get_hierarchy(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<TaskHierarchyResponse>, ServiceError> {
        // Step 1: Draft + Active なルートタスク（親なし）を取得
        let root_tasks = tasks::table
            .filter(tasks::parent_id.is_null())
            .filter(tasks::status.eq("draft").or(tasks::status.eq("active")))
            .order(tasks::created_at.desc())
            .load::<Task>(conn)?;

        // Step 2: 各ルートタスクに対して子タスクを取得し、階層構造を構築
        let hierarchy: Result<Vec<TaskHierarchyResponse>, ServiceError> = root_tasks
            .into_iter()
            .map(|parent_task| {
                // 親タスクのタグを取得
                let parent_tags = task_tags::table
                    .inner_join(tags::table)
                    .filter(task_tags::task_id.eq(&parent_task.id))
                    .select(tags::name)
                    .load::<String>(conn)?;

                // 子タスクを取得（Draft, Active, Completed のみ）
                let child_tasks = tasks::table
                    .filter(tasks::parent_id.eq(&parent_task.id))
                    .filter(
                        tasks::status
                            .eq("draft")
                            .or(tasks::status.eq("active"))
                            .or(tasks::status.eq("completed")),
                    )
                    .order(tasks::created_at.desc())
                    .load::<Task>(conn)?;

                // 子タスクを TaskHierarchyResponse に変換
                let children: Result<Vec<TaskHierarchyResponse>, ServiceError> = child_tasks
                    .into_iter()
                    .map(|child_task| {
                        // 子タスクのタグを取得
                        let child_tags = task_tags::table
                            .inner_join(tags::table)
                            .filter(task_tags::task_id.eq(&child_task.id))
                            .select(tags::name)
                            .load::<String>(conn)?;

                        Ok(TaskHierarchyResponse {
                            id: child_task.id,
                            title: child_task.title,
                            description: child_task.description,
                            status: TaskStatus::from_str(&child_task.status)
                                .ok_or_else(|| {
                                    ServiceError::InvalidTaskStatus(format!(
                                        "Invalid status: {}",
                                        child_task.status
                                    ))
                                })?,
                            tags: child_tags,
                            created_at: child_task.created_at,
                            updated_at: child_task.updated_at,
                            parent_id: child_task.parent_id,
                            children: Vec::new(), // 孫タスク禁止（BR-016）
                        })
                    })
                    .collect();

                Ok(TaskHierarchyResponse {
                    id: parent_task.id,
                    title: parent_task.title,
                    description: parent_task.description,
                    status: TaskStatus::from_str(&parent_task.status).ok_or_else(|| {
                        ServiceError::InvalidTaskStatus(format!(
                            "Invalid status: {}",
                            parent_task.status
                        ))
                    })?,
                    tags: parent_tags,
                    created_at: parent_task.created_at,
                    updated_at: parent_task.updated_at,
                    parent_id: parent_task.parent_id,
                    children: children?,
                })
            })
            .collect();

        hierarchy
    }

    /// タスク検索（フィルタ・キーワード対応）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `params` - 検索パラメータ（q: キーワード、status: ステータス、tags: タグフィルタ）
    ///
    /// # Returns
    /// * `Ok(Vec<TaskResponse>)` - 検索結果のタスクリスト（タグ・子タスクID含む）
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Search Logic
    /// - q: タイトル・説明文のLIKE検索（部分一致）
    /// - status: ステータスフィルタ（未指定時はarchived以外）
    /// - tags: タグ名のOR条件フィルタ
    /// - 全パラメータは任意かつ組み合わせ可能
    pub fn search_tasks(
        conn: &mut SqliteConnection,
        params: SearchTasksParams,
    ) -> Result<Vec<TaskResponse>, ServiceError> {
        // ベースクエリ（boxedで条件分岐可能に）
        let mut query = tasks::table.into_boxed();

        // キーワード検索（タイトル OR 説明文）
        if let Some(keyword) = params.q {
            if !keyword.trim().is_empty() {
                let pattern = format!("%{}%", keyword);
                // パターンをクローンして所有権を渡す
                query = query.filter(
                    tasks::title
                        .like(pattern.clone())
                        .or(tasks::description.like(pattern)),
                );
            }
        }

        // ステータスフィルタ
        if let Some(status) = params.status {
            query = query.filter(tasks::status.eq(status));
        } else {
            // デフォルト: archived以外
            query = query.filter(tasks::status.ne("archived"));
        }

        // タグフィルタ（OR条件）
        if let Some(tag_names) = params.tags {
            if !tag_names.is_empty() {
                // タグ名からタグIDを取得
                let tag_ids: Vec<String> = tags::table
                    .filter(tags::name.eq_any(&tag_names))
                    .select(tags::id)
                    .load::<String>(conn)?;

                if !tag_ids.is_empty() {
                    // task_tagsでフィルタ（サブクエリ使用）
                    // tag_idsをクローンして所有権を渡す
                    query = query.filter(
                        tasks::id.eq_any(
                            task_tags::table
                                .filter(task_tags::tag_id.eq_any(tag_ids))
                                .select(task_tags::task_id),
                        ),
                    );
                } else {
                    // タグが見つからない場合は空の結果を返す
                    return Ok(Vec::new());
                }
            }
        }

        // クエリ実行
        let found_tasks = query.order(tasks::created_at.desc()).load::<Task>(conn)?;

        // 各タスクにタグと子タスクIDを追加（共通ヘルパー使用）
        found_tasks
            .into_iter()
            .map(|task| Self::enrich_task_response(conn, task))
            .collect()
    }

    /// タスクIDのみを検索（軽量版）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `tag_names` - タグ名リスト（OR条件）
    /// * `status_filter` - ステータスフィルタ（オプション、デフォルト: draft + active + completed）
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - タスクIDのリスト
    /// * `Err(ServiceError)` - データベースエラー
    ///
    /// # Usage
    /// - タグフィルター時のパフォーマンス最適化
    /// - フルオブジェクトが不要な場合の軽量検索
    /// - get_hierarchyと同様の検索条件（親: draft+active, 子: draft+active+completed）
    pub fn search_task_ids(
        conn: &mut SqliteConnection,
        tag_names: Option<Vec<String>>,
        status_filter: Option<String>,
    ) -> Result<Vec<String>, ServiceError> {
        let mut query = tasks::table.into_boxed();

        // ステータスフィルタ
        if let Some(status) = status_filter {
            query = query.filter(tasks::status.eq(status));
        } else {
            // デフォルト: get_hierarchyと同じロジック
            // - 親タスク（parent_id IS NULL）: draft OR active
            // - 子タスク（parent_id IS NOT NULL）: draft OR active OR completed
            query = query.filter(
                // 親タスク（draft OR active）
                (tasks::parent_id.is_null()
                    .and(tasks::status.eq("draft").or(tasks::status.eq("active"))))
                    .or(
                        // 子タスク（draft OR active OR completed）
                        tasks::parent_id.is_not_null().and(
                            tasks::status
                                .eq("draft")
                                .or(tasks::status.eq("active"))
                                .or(tasks::status.eq("completed")),
                        ),
                    ),
            );
        }

        // タグフィルタ（OR条件）
        if let Some(tags) = tag_names {
            if !tags.is_empty() {
                // タグ名からタグIDを取得
                let tag_ids: Vec<String> = tags::table
                    .filter(tags::name.eq_any(&tags))
                    .select(tags::id)
                    .load::<String>(conn)?;

                if !tag_ids.is_empty() {
                    // task_tagsでフィルタ
                    query = query.filter(
                        tasks::id.eq_any(
                            task_tags::table
                                .filter(task_tags::tag_id.eq_any(tag_ids))
                                .select(task_tags::task_id),
                        ),
                    );
                } else {
                    // タグが見つからない場合は空の結果を返す
                    return Ok(Vec::new());
                }
            }
        }

        // タスクIDのみを取得
        let task_ids = query.select(tasks::id).load::<String>(conn)?;

        Ok(task_ids)
    }

    /// タスクをIDで取得
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    ///
    /// # Returns
    /// * `Ok(TaskResponse)` - 取得されたタスク（タグ含む）
    /// * `Err(ServiceError)` - タスクが見つからない場合
    pub fn get_task(
        conn: &mut SqliteConnection,
        task_id: &str,
    ) -> Result<TaskResponse, ServiceError> {
        // タスクを取得
        let task = tasks::table
            .find(task_id)
            .first::<Task>(conn)
            .optional()?
            .ok_or_else(|| ServiceError::TaskNotFound(task_id.to_string()))?;

        // タスクに関連付けられたタグ名を取得
        let tag_names = task_tags::table
            .inner_join(tags::table)
            .filter(task_tags::task_id.eq(task_id))
            .select(tags::name)
            .load::<String>(conn)?;

        // 子タスクのIDリストを取得
        let children_ids = tasks::table
            .filter(tasks::parent_id.eq(task_id))
            .select(tasks::id)
            .load::<String>(conn)?;

        let mut response = task.with_tags(tag_names);
        response.children_ids = children_ids; // 子タスクIDを設定
        Ok(response)
    }

    /// タスクを更新
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    /// * `req` - 更新リクエスト
    ///
    /// # Returns
    /// * `Ok(TaskResponse)` - 更新されたタスク（タグ含む）
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Validation
    /// - タイトルが空でないこと（指定されている場合）
    /// - 親タスクが存在すること（変更する場合）
    /// - 循環参照が発生しないこと（親タスク変更時）
    pub fn update_task(
        conn: &mut SqliteConnection,
        task_id: &str,
        req_input: UpdateTaskRequestInput,
    ) -> Result<TaskResponse, ServiceError> {
        // タスクが存在するか確認
        let existing_task = tasks::table
            .find(task_id)
            .first::<Task>(conn)
            .optional()?
            .ok_or_else(|| ServiceError::TaskNotFound(task_id.to_string()))?;

        // Draft状態チェック: Draft以外のタスクは編集不可（REQ-0016）
        if existing_task.status != "draft" {
            return Err(ServiceError::TaskNotDraft(task_id.to_string()));
        }

        // バリデーション: タイトルが空でないこと
        if let Some(ref title) = req_input.title {
            if title.trim().is_empty() {
                return Err(ServiceError::InvalidInput(
                    "Title cannot be empty".to_string(),
                ));
            }
        }

        // バリデーション: 親タスクが存在するか確認（指定されている場合）
        if let Some(ref parent_id) = req_input.parent_id {
            let parent_exists = tasks::table
                .find(parent_id)
                .select(tasks::id)
                .first::<String>(conn)
                .optional()?;

            if parent_exists.is_none() {
                return Err(ServiceError::ParentTaskNotFound(parent_id.clone()));
            }

            // 【新規追加】階層深度チェック（孫タスク作成禁止 - BR-016）
            Self::validate_hierarchy_depth(conn, parent_id)?;

            // 循環参照チェック: 親タスクを変更する場合
            if Self::would_create_cycle(conn, task_id, parent_id)? {
                return Err(ServiceError::CircularDependency(task_id.to_string()));
            }
        }

        // UpdateTaskRequestInputからUpdateTaskRequestへ変換
        let mut req = UpdateTaskRequest {
            title: req_input.title,
            description: req_input.description,
            status: req_input.status,
            parent_id: req_input.parent_id,
            updated_at: None,
        };

        // updated_atタイムスタンプを設定
        req = req.with_timestamp();

        // タスク更新
        diesel::update(tasks::table.find(task_id))
            .set(&req)
            .execute(conn)?;

        // タグの更新（tags配列が指定されている場合）
        if let Some(ref new_tags) = req_input.tags {
            // 既存のタグ関連付けをすべて削除
            diesel::delete(task_tags::table.filter(task_tags::task_id.eq(task_id)))
                .execute(conn)?;

            // 新しいタグを関連付け
            if !new_tags.is_empty() {
                for tag_name in new_tags {
                    // タグIDを取得
                    let tag_id_result = tags::table
                        .filter(tags::name.eq(tag_name))
                        .select(tags::id)
                        .first::<String>(conn)
                        .optional()?;

                    if let Some(tag_id) = tag_id_result {
                        // task_tags挿入
                        diesel::insert_into(task_tags::table)
                            .values((
                                task_tags::task_id.eq(task_id),
                                task_tags::tag_id.eq(&tag_id),
                            ))
                            .execute(conn)?;
                    } else {
                        return Err(ServiceError::TagNotFound(tag_name.clone()));
                    }
                }
            }
        }

        // 【新規追加】親ステータス更新（BR-013: 子タスク変更時の親ステータス自動同期）
        Self::update_parent_status_if_needed(conn, task_id)?;

        // 更新されたタスクを取得して返却
        Self::get_task(conn, task_id)
    }

    /// タスクを削除（論理削除: archivedステータスに変更）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    ///
    /// # Returns
    /// * `Ok(())` - 削除成功
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Validation
    /// - 子タスクを持つタスクは削除不可
    pub fn delete_task(
        conn: &mut SqliteConnection,
        task_id: &str,
    ) -> Result<(), ServiceError> {
        // タスクが存在するか確認
        let existing_task = tasks::table
            .find(task_id)
            .first::<Task>(conn)
            .optional()?
            .ok_or_else(|| ServiceError::TaskNotFound(task_id.to_string()))?;

        // Draft状態チェック: Draft以外のタスクは削除不可（REQ-0017）
        if existing_task.status != "draft" {
            return Err(ServiceError::TaskNotDraft(task_id.to_string()));
        }

        // 子タスクが存在するか確認
        let has_children = tasks::table
            .filter(tasks::parent_id.eq(task_id))
            .select(tasks::id)
            .first::<String>(conn)
            .optional()?
            .is_some();

        if has_children {
            return Err(ServiceError::TaskHasChildren(task_id.to_string()));
        }

        // 論理削除: archivedステータスに変更
        use crate::models::task::TaskStatus;
        let update_req = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(TaskStatus::Archived.as_str().to_string()),
            parent_id: None,
            updated_at: None,
        }
        .with_timestamp();

        diesel::update(tasks::table.find(task_id))
            .set(&update_req)
            .execute(conn)?;

        // 【新規追加】親ステータス更新（BR-013: 子タスク削除時の親ステータス自動同期）
        Self::update_parent_status_if_needed(conn, task_id)?;

        Ok(())
    }

    /// タスクをデータベースから完全に削除する（物理削除）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    ///
    /// # Returns
    /// * `Ok(())` - 成功
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Validation
    /// - Archivedステータスのタスクのみ物理削除可能（REQ-0018）
    ///
    /// # Notes
    /// - 子タスクも自動的に削除される（ON DELETE CASCADE）
    /// - task_tagsとtask_queueの関連レコードもCASCADEで自動削除される
    pub fn delete_task_permanently(
        conn: &mut SqliteConnection,
        task_id: &str,
    ) -> Result<(), ServiceError> {
        // タスクが存在するか確認
        let existing_task = tasks::table
            .find(task_id)
            .first::<Task>(conn)
            .optional()?
            .ok_or_else(|| ServiceError::TaskNotFound(task_id.to_string()))?;

        // Archived状態チェック: Archived以外のタスクは物理削除不可（REQ-0018）
        if existing_task.status != "archived" {
            return Err(ServiceError::TaskNotArchived(task_id.to_string()));
        }

        // 物理削除: データベースから完全に削除
        // ON DELETE CASCADEにより、子タスクも自動的に削除される
        diesel::delete(tasks::table.find(task_id)).execute(conn)?;

        Ok(())
    }

    /// Archivedタスクを復元してDraft状態に戻す
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    ///
    /// # Returns
    /// * `Ok(TaskResponse)` - 復元されたタスク
    /// * `Err(ServiceError)` - エラー
    ///
    /// # Validation
    /// - Archivedステータスのタスクのみ復元可能（REQ-0022）
    ///
    /// # Notes
    /// - タスクのステータスをArchivedからDraftに変更する
    /// - updated_atタイムスタンプを更新する
    pub fn restore_task(
        conn: &mut SqliteConnection,
        task_id: &str,
    ) -> Result<TaskResponse, ServiceError> {
        // タスクが存在するか確認
        let existing_task = tasks::table
            .find(task_id)
            .first::<Task>(conn)
            .optional()?
            .ok_or_else(|| ServiceError::TaskNotFound(task_id.to_string()))?;

        // Archived状態チェック: Archived以外のタスクは復元不可（REQ-0022）
        if existing_task.status != "archived" {
            return Err(ServiceError::TaskNotArchived(task_id.to_string()));
        }

        // Draft状態に変更し、updated_atタイムスタンプを更新
        let now = Utc::now().to_rfc3339();
        diesel::update(tasks::table.find(task_id))
            .set((
                tasks::status.eq("draft"),
                tasks::updated_at.eq(&now),
            ))
            .execute(conn)?;

        // 更新されたタスクを取得して返却
        Self::get_task(conn, task_id)
    }

    /// タスクレスポンスをエンリッチ（タグ・子タスクID・親タイトル付与）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task` - エンリッチ対象のタスク
    /// * `parent_title` - 親タスクのタイトル（バッチ取得済みの場合に渡す）
    ///
    /// # Returns
    /// * `Ok(TaskResponse)` - エンリッチされたTaskResponse
    /// * `Err(ServiceError)` - データベースエラー
    ///
    /// # Notes
    /// - list_tasks_paginatedではバッチ取得したparent_titleを渡す
    /// - その他のAPIでは個別にDB取得
    fn enrich_task_response_with_parent_title(
        conn: &mut SqliteConnection,
        task: Task,
        parent_title: Option<String>,
    ) -> Result<TaskResponse, ServiceError> {
        // タグ名を取得
        let tag_names = task_tags::table
            .inner_join(tags::table)
            .filter(task_tags::task_id.eq(&task.id))
            .select(tags::name)
            .load::<String>(conn)?;

        // 子タスクのIDリストを取得
        let children_ids = tasks::table
            .filter(tasks::parent_id.eq(&task.id))
            .select(tasks::id)
            .load::<String>(conn)?;

        let mut response = task.with_tags(tag_names);
        response.children_ids = children_ids;
        response.parent_title = parent_title;
        Ok(response)
    }

    /// タスクレスポンスをエンリッチ（互換性維持用ラッパー）
    ///
    /// # Notes
    /// - list_tasks, search_tasksなど既存APIで使用
    /// - parent_titleは個別にDB取得
    fn enrich_task_response(
        conn: &mut SqliteConnection,
        task: Task,
    ) -> Result<TaskResponse, ServiceError> {
        // 親タスクのタイトルを個別に取得
        let parent_title = if let Some(ref parent_id) = task.parent_id {
            tasks::table
                .filter(tasks::id.eq(parent_id))
                .select(tasks::title)
                .first::<String>(conn)
                .optional()?
        } else {
            None
        };

        Self::enrich_task_response_with_parent_title(conn, task, parent_title)
    }

    /// 指定されたタスクが子タスクを持つかどうかを確認
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - タスクID
    ///
    /// # Returns
    /// * `Ok(true)` - 子タスクが存在する
    /// * `Ok(false)` - 子タスクが存在しない
    /// * `Err(ServiceError)` - データベースエラー
    ///
    /// # Usage
    /// - キュー登録制限（親タスクはキューに追加不可）
    /// - 親子ステータス自動同期（子タスクの有無確認）
    pub fn has_children(
        conn: &mut SqliteConnection,
        task_id: &str,
    ) -> Result<bool, ServiceError> {
        let count = tasks::table
            .filter(tasks::parent_id.eq(task_id))
            .count()
            .get_result::<i64>(conn)?;

        Ok(count > 0)
    }

    /// 循環参照が発生するかチェック
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - チェック対象のタスクID
    /// * `new_parent_id` - 新しい親タスクID
    ///
    /// # Returns
    /// * `Ok(true)` - 循環参照が発生する
    /// * `Ok(false)` - 循環参照が発生しない
    fn would_create_cycle(
        conn: &mut SqliteConnection,
        task_id: &str,
        new_parent_id: &str,
    ) -> Result<bool, ServiceError> {
        // 自分自身を親にすることはできない
        if task_id == new_parent_id {
            return Ok(true);
        }

        // 祖先を辿って循環参照をチェック
        let mut current_id = new_parent_id.to_string();
        let mut visited = std::collections::HashSet::new();
        visited.insert(task_id.to_string());

        loop {
            if visited.contains(&current_id) {
                return Ok(true); // 循環参照を検出
            }
            visited.insert(current_id.clone());

            // 親タスクを取得
            let parent = tasks::table
                .find(&current_id)
                .select(tasks::parent_id)
                .first::<Option<String>>(conn)
                .optional()?;

            match parent {
                Some(Some(parent_id)) => {
                    current_id = parent_id;
                }
                _ => break, // 親がいない、またはタスクが見つからない
            }
        }

        Ok(false)
    }

    /// 階層深度バリデーション（孫タスク作成禁止）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `parent_id` - 親タスクID
    ///
    /// # Returns
    /// * `Ok(())` - 階層深度が許容範囲内（親タスク自身が親を持たない）
    /// * `Err(ServiceError::GrandchildNotAllowed)` - 親タスクが既に子タスクである（孫タスク作成不可）
    /// * `Err(ServiceError::TaskNotFound)` - 親タスクが存在しない
    ///
    /// # Constraint
    /// - BR-016: タスクの階層は最大2レベル（親-子）までとする
    fn validate_hierarchy_depth(
        conn: &mut SqliteConnection,
        parent_id: &str,
    ) -> Result<(), ServiceError> {
        // 親タスク自身が親を持っているかチェック
        let parent_task = tasks::table
            .find(parent_id)
            .first::<Task>(conn)
            .optional()?
            .ok_or_else(|| ServiceError::TaskNotFound(parent_id.to_string()))?;

        if parent_task.parent_id.is_some() {
            return Err(ServiceError::GrandchildNotAllowed);
        }

        Ok(())
    }

    /// 子タスクのステータスから親タスクのステータスを計算
    ///
    /// # Arguments
    /// * `child_statuses` - 子タスクのステータス一覧
    ///
    /// # Returns
    /// * `TaskStatus` - 計算された親タスクのステータス
    ///
    /// # Business Rules (BR-013)
    /// - 全子が Draft → 親も Draft
    /// - 1つでも Active → 親も Active
    /// - 全子が Completed → 親も Completed
    /// - 全子が (Archived OR Completed) → 親も Completed
    /// - 混在状態（Draft + Completed など）→ Active とみなす
    fn calculate_parent_status(child_statuses: Vec<TaskStatus>) -> TaskStatus {
        if child_statuses.is_empty() {
            // 子タスクがない場合は Draft（通常は呼び出し側で処理）
            return TaskStatus::Draft;
        }

        // 1つでもActiveがあれば親もActive
        if child_statuses.iter().any(|s| matches!(s, TaskStatus::Active)) {
            return TaskStatus::Active;
        }

        // 全てがCompletedなら親もCompleted
        if child_statuses.iter().all(|s| matches!(s, TaskStatus::Completed)) {
            return TaskStatus::Completed;
        }

        // 全てがDraftなら親もDraft
        if child_statuses.iter().all(|s| matches!(s, TaskStatus::Draft)) {
            return TaskStatus::Draft;
        }

        // 全てが(Archived OR Completed)なら親もCompleted
        if child_statuses.iter().all(|s| {
            matches!(s, TaskStatus::Completed | TaskStatus::Archived)
        }) {
            return TaskStatus::Completed;
        }

        // 混在状態（Draft + Completed など）→ Activeとみなす
        TaskStatus::Active
    }

    /// 親タスクのステータスを子タスクに基づいて更新（再帰的）
    ///
    /// # Arguments
    /// * `conn` - データベース接続
    /// * `task_id` - 子タスクID（この子の親を更新する）
    ///
    /// # Returns
    /// * `Ok(())` - 更新成功または親タスクが存在しない
    /// * `Err(ServiceError)` - データベースエラー
    ///
    /// # Behavior
    /// 1. 指定されたタスクの親タスクIDを取得
    /// 2. 親タスクの全子タスクのステータスを集計
    /// 3. BR-013に基づき親タスクのステータスを計算
    /// 4. 親タスクのステータスを更新
    /// 5. 再帰的に祖父タスクも更新（BR-016により実質1階層のみ）
    pub fn update_parent_status_if_needed(
        conn: &mut SqliteConnection,
        task_id: &str,
    ) -> Result<(), ServiceError> {
        // 1. 親タスクIDを取得
        let parent_task_id: Option<String> = tasks::table
            .filter(tasks::id.eq(task_id))
            .select(tasks::parent_id)
            .first::<Option<String>>(conn)
            .optional()?
            .flatten();

        if let Some(parent_id_value) = parent_task_id {
            // 2. 親タスクの全子タスクのステータスを取得
            let child_statuses: Vec<TaskStatus> = tasks::table
                .filter(tasks::parent_id.eq(&parent_id_value))
                .select(tasks::status)
                .load::<String>(conn)?
                .into_iter()
                .map(|s| {
                    TaskStatus::from_str(&s)
                        .ok_or_else(|| ServiceError::InvalidTaskStatus(s.to_string()))
                })
                .collect::<Result<Vec<_>, _>>()?;

            // 3. 親タスクのステータスを計算
            let new_parent_status = Self::calculate_parent_status(child_statuses);

            // 4. 親タスクのステータスとupdated_atを更新
            let now = chrono::Utc::now().to_rfc3339();
            diesel::update(tasks::table.filter(tasks::id.eq(&parent_id_value)))
                .set((
                    tasks::status.eq(new_parent_status.as_str()),
                    tasks::updated_at.eq(now),
                ))
                .execute(conn)?;

            // 5. 再帰的に祖父タスクも更新（BR-016により孫タスク禁止のため、通常は実行されない）
            Self::update_parent_status_if_needed(conn, &parent_id_value)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TaskStatus;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    fn setup_test_db() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:")
            .expect("Failed to create in-memory database");

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");

        conn
    }

    #[test]
    fn test_create_task_success() {
        let mut conn = setup_test_db();

        let req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: Some("Test description".to_string()),
            tags: vec![],
            parent_id: None,
        };

        let result = TaskService::create_task(&mut conn, req);
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, Some("Test description".to_string()));
        assert_eq!(task.status, TaskStatus::Draft);
        assert_eq!(task.tags, Vec::<String>::new());
    }

    #[test]
    fn test_create_task_empty_title() {
        let mut conn = setup_test_db();

        let req = CreateTaskRequest {
            title: "   ".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };

        let result = TaskService::create_task(&mut conn, req);
        assert!(result.is_err());

        if let Err(ServiceError::InvalidInput(msg)) = result {
            assert_eq!(msg, "Title cannot be empty");
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_create_task_parent_not_found() {
        let mut conn = setup_test_db();

        let req = CreateTaskRequest {
            title: "Child Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some("non-existent-id".to_string()),
        };

        let result = TaskService::create_task(&mut conn, req);
        assert!(result.is_err());

        if let Err(ServiceError::ParentTaskNotFound(id)) = result {
            assert_eq!(id, "non-existent-id");
        } else {
            panic!("Expected ParentTaskNotFound error");
        }
    }

    #[test]
    fn test_create_task_with_valid_parent() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // 子タスク作成
        let child_req = CreateTaskRequest {
            title: "Child Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let result = TaskService::create_task(&mut conn, child_req);
        assert!(result.is_ok());

        let child = result.unwrap();
        assert_eq!(child.parent_id, Some(parent.id));
    }

    #[test]
    fn test_grandchild_creation_rejected() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // 子タスク作成
        let child_req = CreateTaskRequest {
            title: "Child Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let child = TaskService::create_task(&mut conn, child_req).unwrap();

        // 孫タスク作成を試みる → エラー
        let grandchild_req = CreateTaskRequest {
            title: "Grandchild Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(child.id.clone()),
        };
        let result = TaskService::create_task(&mut conn, grandchild_req);
        assert!(result.is_err());

        if let Err(ServiceError::GrandchildNotAllowed) = result {
            // 期待通りのエラー
        } else {
            panic!("Expected GrandchildNotAllowed error");
        }
    }

    #[test]
    fn test_grandchild_update_rejected() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // 子タスク作成
        let child_req = CreateTaskRequest {
            title: "Child Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let child = TaskService::create_task(&mut conn, child_req).unwrap();

        // 通常のタスク作成
        let task_req = CreateTaskRequest {
            title: "Normal Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, task_req).unwrap();

        // 通常のタスクの親を子タスクに変更しようとする → エラー
        let update_req = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: None,
            parent_id: Some(child.id.clone()),
            tags: None,
        };
        let result = TaskService::update_task(&mut conn, &task.id, update_req);
        assert!(result.is_err());

        if let Err(ServiceError::GrandchildNotAllowed) = result {
            // 期待通りのエラー
        } else {
            panic!("Expected GrandchildNotAllowed error, got {:?}", result);
        }
    }

    #[test]
    fn test_normal_child_creation_allowed() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // 子タスク作成 → 成功
        let child_req = CreateTaskRequest {
            title: "Child Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let result = TaskService::create_task(&mut conn, child_req);
        assert!(result.is_ok());

        let child = result.unwrap();
        assert_eq!(child.parent_id, Some(parent.id));
        assert_eq!(child.title, "Child Task");
    }

    #[test]
    fn test_has_children_true() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // 子タスク作成
        let child_req = CreateTaskRequest {
            title: "Child Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        TaskService::create_task(&mut conn, child_req).unwrap();

        // has_children チェック → true
        let result = TaskService::has_children(&mut conn, &parent.id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_has_children_false() {
        let mut conn = setup_test_db();

        // 親タスク作成（子タスクなし）
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // has_children チェック → false
        let result = TaskService::has_children(&mut conn, &parent.id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_calculate_status_all_draft() {
        let statuses = vec![TaskStatus::Draft, TaskStatus::Draft];
        let result = TaskService::calculate_parent_status(statuses);
        assert_eq!(result, TaskStatus::Draft);
    }

    #[test]
    fn test_calculate_status_one_active() {
        let statuses = vec![TaskStatus::Active];
        let result = TaskService::calculate_parent_status(statuses);
        assert_eq!(result, TaskStatus::Active);
    }

    #[test]
    fn test_calculate_status_draft_and_active() {
        let statuses = vec![TaskStatus::Draft, TaskStatus::Active];
        let result = TaskService::calculate_parent_status(statuses);
        assert_eq!(result, TaskStatus::Active);
    }

    #[test]
    fn test_calculate_status_all_completed() {
        let statuses = vec![TaskStatus::Completed, TaskStatus::Completed];
        let result = TaskService::calculate_parent_status(statuses);
        assert_eq!(result, TaskStatus::Completed);
    }

    #[test]
    fn test_calculate_status_completed_archived() {
        let statuses = vec![TaskStatus::Completed, TaskStatus::Archived];
        let result = TaskService::calculate_parent_status(statuses);
        assert_eq!(result, TaskStatus::Completed);
    }

    #[test]
    fn test_calculate_status_mixed() {
        // Draft + Completed の混在 → Active
        let statuses = vec![TaskStatus::Draft, TaskStatus::Completed];
        let result = TaskService::calculate_parent_status(statuses);
        assert_eq!(result, TaskStatus::Active);
    }

    #[test]
    fn test_update_parent_one_level() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();
        assert_eq!(parent.status, TaskStatus::Draft);

        // 子タスク1作成（Draft）
        let child1_req = CreateTaskRequest {
            title: "Child Task 1".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let child1 = TaskService::create_task(&mut conn, child1_req).unwrap();

        // 子タスク2作成（Draft）
        let child2_req = CreateTaskRequest {
            title: "Child Task 2".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let _child2 = TaskService::create_task(&mut conn, child2_req).unwrap();

        // 子タスク1をActiveに変更
        let update_req = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(TaskStatus::Active.as_str().to_string()),
            parent_id: None,
            updated_at: None,
        };
        diesel::update(tasks::table.filter(tasks::id.eq(&child1.id)))
            .set(&update_req)
            .execute(&mut conn)
            .unwrap();

        // 親ステータス更新を呼び出し
        TaskService::update_parent_status_if_needed(&mut conn, &child1.id).unwrap();

        // 親タスクのステータスを確認 → Active（1つでもActiveがあるため）
        let updated_parent = TaskService::get_task(&mut conn, &parent.id).unwrap();
        assert_eq!(updated_parent.status, TaskStatus::Active);
    }

    #[test]
    fn test_update_parent_no_parent() {
        let mut conn = setup_test_db();

        // 親なしタスク作成
        let task_req = CreateTaskRequest {
            title: "Task without parent".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, task_req).unwrap();

        // 親ステータス更新を呼び出し（親がいないので何もしない）
        let result = TaskService::update_parent_status_if_needed(&mut conn, &task.id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_task_success() {
        let mut conn = setup_test_db();

        // タスク作成
        let req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: Some("Description".to_string()),
            tags: vec![],
            parent_id: None,
        };
        let created = TaskService::create_task(&mut conn, req).unwrap();

        // タスク取得
        let result = TaskService::get_task(&mut conn, &created.id);
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.id, created.id);
        assert_eq!(task.title, "Test Task");
    }

    #[test]
    fn test_get_task_not_found() {
        let mut conn = setup_test_db();

        let result = TaskService::get_task(&mut conn, "non-existent-id");
        assert!(result.is_err());

        if let Err(ServiceError::TaskNotFound(id)) = result {
            assert_eq!(id, "non-existent-id");
        } else {
            panic!("Expected TaskNotFound error");
        }
    }

    #[test]
    fn test_update_task_success() {
        let mut conn = setup_test_db();

        // タスク作成
        let req = CreateTaskRequest {
            title: "Original Title".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let created = TaskService::create_task(&mut conn, req).unwrap();

        // タスク更新
        let update_req = UpdateTaskRequestInput {
            title: Some("Updated Title".to_string()),
            description: Some("New description".to_string()),
            status: None,
            parent_id: None,
            tags: None,
        };
        let result = TaskService::update_task(&mut conn, &created.id, update_req);
        assert!(result.is_ok());

        let updated = result.unwrap();
        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.description, Some("New description".to_string()));
    }

    #[test]
    fn test_update_task_not_found() {
        let mut conn = setup_test_db();

        let update_req = UpdateTaskRequestInput {
            title: Some("New Title".to_string()),
            description: None,
            status: None,
            parent_id: None,
            tags: None,
        };
        let result = TaskService::update_task(&mut conn, "non-existent-id", update_req);
        assert!(result.is_err());

        if let Err(ServiceError::TaskNotFound(id)) = result {
            assert_eq!(id, "non-existent-id");
        } else {
            panic!("Expected TaskNotFound error");
        }
    }

    #[test]
    fn test_update_task_empty_title() {
        let mut conn = setup_test_db();

        // タスク作成
        let req = CreateTaskRequest {
            title: "Original Title".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let created = TaskService::create_task(&mut conn, req).unwrap();

        // 空のタイトルで更新を試みる
        let update_req = UpdateTaskRequestInput {
            title: Some("   ".to_string()),
            description: None,
            status: None,
            parent_id: None,
            tags: None,
        };
        let result = TaskService::update_task(&mut conn, &created.id, update_req);
        assert!(result.is_err());

        if let Err(ServiceError::InvalidInput(msg)) = result {
            assert_eq!(msg, "Title cannot be empty");
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_delete_task_success() {
        let mut conn = setup_test_db();

        // タスク作成
        let req = CreateTaskRequest {
            title: "Task to Delete".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let created = TaskService::create_task(&mut conn, req).unwrap();

        // タスク削除
        let result = TaskService::delete_task(&mut conn, &created.id);
        assert!(result.is_ok());

        // タスクが論理削除されたか確認
        let deleted_task = TaskService::get_task(&mut conn, &created.id).unwrap();
        assert_eq!(deleted_task.status, TaskStatus::Archived);
    }

    #[test]
    fn test_delete_task_not_found() {
        let mut conn = setup_test_db();

        let result = TaskService::delete_task(&mut conn, "non-existent-id");
        assert!(result.is_err());

        if let Err(ServiceError::TaskNotFound(id)) = result {
            assert_eq!(id, "non-existent-id");
        } else {
            panic!("Expected TaskNotFound error");
        }
    }

    #[test]
    fn test_delete_task_with_children() {
        let mut conn = setup_test_db();

        // 親タスク作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // 子タスク作成
        let child_req = CreateTaskRequest {
            title: "Child Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        TaskService::create_task(&mut conn, child_req).unwrap();

        // 子タスクを持つ親タスクの削除を試みる
        let result = TaskService::delete_task(&mut conn, &parent.id);
        assert!(result.is_err());

        if let Err(ServiceError::TaskHasChildren(id)) = result {
            assert_eq!(id, parent.id);
        } else {
            panic!("Expected TaskHasChildren error");
        }
    }

    #[test]
    fn test_circular_dependency_self_reference() {
        let mut conn = setup_test_db();

        // タスク作成
        let req = CreateTaskRequest {
            title: "Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let created = TaskService::create_task(&mut conn, req).unwrap();

        // 自分自身を親に設定しようとする
        let update_req = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: None,
            parent_id: Some(created.id.clone()),
            tags: None,
        };
        let result = TaskService::update_task(&mut conn, &created.id, update_req);
        assert!(result.is_err());

        if let Err(ServiceError::CircularDependency(id)) = result {
            assert_eq!(id, created.id);
        } else {
            panic!("Expected CircularDependency error");
        }
    }

    #[test]
    fn test_circular_dependency_indirect() {
        let mut conn = setup_test_db();

        // タスクA作成
        let req_a = CreateTaskRequest {
            title: "Task A".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task_a = TaskService::create_task(&mut conn, req_a).unwrap();

        // タスクB作成（親: A）
        let req_b = CreateTaskRequest {
            title: "Task B".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(task_a.id.clone()),
        };
        let task_b = TaskService::create_task(&mut conn, req_b).unwrap();

        // タスクAの親をタスクBに設定しようとする（循環参照: A -> B -> A）
        // 注: BR-016により2レベル制限があるため、2階層での循環参照テスト
        let update_req = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: None,
            parent_id: Some(task_b.id.clone()),
            tags: None,
        };
        let result = TaskService::update_task(&mut conn, &task_a.id, update_req);
        assert!(result.is_err());

        // 階層深度チェックが先に実行されるため、GrandchildNotAllowedエラーになる
        // （タスクBは既に親Aを持っているため、Aを子にすることはできない）
        if let Err(ServiceError::GrandchildNotAllowed) = result {
            // 期待通り：Bは既に子タスクなので、Bを親にすることはできない
        } else {
            panic!("Expected GrandchildNotAllowed error (since B is already a child task), got {:?}", result);
        }
    }

    // ===== 検索機能テスト =====

    #[test]
    fn test_search_tasks_by_keyword() {
        let mut conn = setup_test_db();

        // タスク作成
        let req1 = CreateTaskRequest {
            title: "Rust programming".to_string(),
            description: Some("Learn Rust language".to_string()),
            tags: vec![],
            parent_id: None,
        };
        let req2 = CreateTaskRequest {
            title: "Python coding".to_string(),
            description: Some("Build web app".to_string()),
            tags: vec![],
            parent_id: None,
        };
        TaskService::create_task(&mut conn, req1).unwrap();
        TaskService::create_task(&mut conn, req2).unwrap();

        // キーワード検索: "Rust"
        let params = SearchTasksParams {
            q: Some("Rust".to_string()),
            status: None,
            tags: None,
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Rust programming");

        // キーワード検索: "web"（description内）
        let params = SearchTasksParams {
            q: Some("web".to_string()),
            status: None,
            tags: None,
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Python coding");
    }

    #[test]
    fn test_search_tasks_by_status() {
        let mut conn = setup_test_db();

        // draft タスク作成
        let req1 = CreateTaskRequest {
            title: "Draft Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let _task1 = TaskService::create_task(&mut conn, req1).unwrap();

        // active タスク作成（ステータス更新）
        let req2 = CreateTaskRequest {
            title: "Active Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task2 = TaskService::create_task(&mut conn, req2).unwrap();
        let update_req = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: Some("active".to_string()),
            parent_id: None,
            tags: None,
        };
        TaskService::update_task(&mut conn, &task2.id, update_req).unwrap();

        // ステータス検索: "draft"
        let params = SearchTasksParams {
            q: None,
            status: Some("draft".to_string()),
            tags: None,
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Draft Task");

        // ステータス検索: "active"
        let params = SearchTasksParams {
            q: None,
            status: Some("active".to_string()),
            tags: None,
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Active Task");
    }

    #[test]
    fn test_search_tasks_by_tags() {
        use crate::models::tag::CreateTagRequest;
        use crate::service::TagService;

        let mut conn = setup_test_db();

        // タグ作成
        let tag1_req = CreateTagRequest {
            name: "work".to_string(),
            color: None,
        };
        let tag2_req = CreateTagRequest {
            name: "personal".to_string(),
            color: None,
        };
        TagService::create_tag(&mut conn, tag1_req).unwrap();
        TagService::create_tag(&mut conn, tag2_req).unwrap();

        // タスク作成（タグ付き）
        let req1 = CreateTaskRequest {
            title: "Work Task".to_string(),
            description: None,
            tags: vec!["work".to_string()],
            parent_id: None,
        };
        let req2 = CreateTaskRequest {
            title: "Personal Task".to_string(),
            description: None,
            tags: vec!["personal".to_string()],
            parent_id: None,
        };
        TaskService::create_task(&mut conn, req1).unwrap();
        TaskService::create_task(&mut conn, req2).unwrap();

        // タグ検索: "work"
        let params = SearchTasksParams {
            q: None,
            status: None,
            tags: Some(vec!["work".to_string()]),
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Work Task");

        // タグ検索: "work" OR "personal"
        let params = SearchTasksParams {
            q: None,
            status: None,
            tags: Some(vec!["work".to_string(), "personal".to_string()]),
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_tasks_combined_filters() {
        use crate::models::tag::CreateTagRequest;
        use crate::service::TagService;

        let mut conn = setup_test_db();

        // タグ作成
        let tag_req = CreateTagRequest {
            name: "urgent".to_string(),
            color: None,
        };
        TagService::create_tag(&mut conn, tag_req).unwrap();

        // タスク作成
        let req1 = CreateTaskRequest {
            title: "Urgent Rust Task".to_string(),
            description: Some("Fix bug".to_string()),
            tags: vec!["urgent".to_string()],
            parent_id: None,
        };
        let req2 = CreateTaskRequest {
            title: "Urgent Python Task".to_string(),
            description: Some("Add feature".to_string()),
            tags: vec!["urgent".to_string()],
            parent_id: None,
        };
        TaskService::create_task(&mut conn, req1).unwrap();
        TaskService::create_task(&mut conn, req2).unwrap();

        // 複合検索: キーワード "Rust" AND タグ "urgent"
        let params = SearchTasksParams {
            q: Some("Rust".to_string()),
            status: None,
            tags: Some(vec!["urgent".to_string()]),
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Urgent Rust Task");
    }

    #[test]
    fn test_search_tasks_no_match() {
        let mut conn = setup_test_db();

        // タスク作成
        let req = CreateTaskRequest {
            title: "Sample Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        TaskService::create_task(&mut conn, req).unwrap();

        // 存在しないキーワードで検索
        let params = SearchTasksParams {
            q: Some("NonExistentKeyword".to_string()),
            status: None,
            tags: None,
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 0);

        // 存在しないタグで検索
        let params = SearchTasksParams {
            q: None,
            status: None,
            tags: Some(vec!["nonexistent".to_string()]),
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_tasks_empty_params() {
        let mut conn = setup_test_db();

        // draft タスク作成
        let req1 = CreateTaskRequest {
            title: "Task 1".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        TaskService::create_task(&mut conn, req1).unwrap();

        // archived タスク作成
        let req2 = CreateTaskRequest {
            title: "Task 2".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task2 = TaskService::create_task(&mut conn, req2).unwrap();
        let update_req = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: Some("archived".to_string()),
            parent_id: None,
            tags: None,
        };
        TaskService::update_task(&mut conn, &task2.id, update_req).unwrap();

        // 空のパラメータで検索（archived以外を取得）
        let params = SearchTasksParams {
            q: None,
            status: None,
            tags: None,
        };
        let results = TaskService::search_tasks(&mut conn, params).unwrap();
        assert_eq!(results.len(), 1); // archived以外
        assert_eq!(results[0].title, "Task 1");
    }

    #[test]
    fn test_get_hierarchy_includes_completed_children() {
        let mut conn = setup_test_db();

        // 親タスク（Draft）を作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // 子タスク1（Active）を作成
        let child1_req = CreateTaskRequest {
            title: "Child Task 1".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let child1 = TaskService::create_task(&mut conn, child1_req).unwrap();
        let update_req1 = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: Some("active".to_string()),
            parent_id: None,
            tags: None,
        };
        TaskService::update_task(&mut conn, &child1.id, update_req1).unwrap();

        // 子タスク2（Completed）を作成
        let child2_req = CreateTaskRequest {
            title: "Child Task 2".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let child2 = TaskService::create_task(&mut conn, child2_req).unwrap();
        let update_req2 = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: Some("completed".to_string()),
            parent_id: None,
            tags: None,
        };
        TaskService::update_task(&mut conn, &child2.id, update_req2).unwrap();

        // get_hierarchy で取得
        let hierarchy = TaskService::get_hierarchy(&mut conn).unwrap();

        // 親タスク1つが取得されることを確認
        assert_eq!(hierarchy.len(), 1);
        assert_eq!(hierarchy[0].title, "Parent Task");

        // 子タスク2つ（Active + Completed）が含まれることを確認
        assert_eq!(hierarchy[0].children.len(), 2);

        // 子タスクのステータスを確認
        let child_statuses: Vec<TaskStatus> = hierarchy[0]
            .children
            .iter()
            .map(|c| c.status.clone())
            .collect();
        assert!(child_statuses.contains(&TaskStatus::Active));
        assert!(child_statuses.contains(&TaskStatus::Completed));
    }

    #[test]
    fn test_get_hierarchy_excludes_completed_parent() {
        let mut conn = setup_test_db();

        // 親タスク（Completed）を作成
        let parent_req = CreateTaskRequest {
            title: "Completed Parent".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();
        let update_req = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: Some("completed".to_string()),
            parent_id: None,
            tags: None,
        };
        TaskService::update_task(&mut conn, &parent.id, update_req).unwrap();

        // 子タスク（Draft）を作成
        let child_req = CreateTaskRequest {
            title: "Draft Child".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        TaskService::create_task(&mut conn, child_req).unwrap();

        // get_hierarchy で取得
        let hierarchy = TaskService::get_hierarchy(&mut conn).unwrap();

        // Completed な親タスクは除外される
        assert_eq!(hierarchy.len(), 0);
    }

    #[test]
    fn test_get_hierarchy_excludes_archived_children() {
        let mut conn = setup_test_db();

        // 親タスク（Active）を作成
        let parent_req = CreateTaskRequest {
            title: "Active Parent".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();
        let update_parent_req = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: Some("active".to_string()),
            parent_id: None,
            tags: None,
        };
        TaskService::update_task(&mut conn, &parent.id, update_parent_req).unwrap();

        // 子タスク1（Draft）を作成
        let child1_req = CreateTaskRequest {
            title: "Draft Child".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        TaskService::create_task(&mut conn, child1_req).unwrap();

        // 子タスク2（Archived）を作成
        let child2_req = CreateTaskRequest {
            title: "Archived Child".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let child2 = TaskService::create_task(&mut conn, child2_req).unwrap();
        let update_req2 = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: Some("archived".to_string()),
            parent_id: None,
            tags: None,
        };
        TaskService::update_task(&mut conn, &child2.id, update_req2).unwrap();

        // get_hierarchy で取得
        let hierarchy = TaskService::get_hierarchy(&mut conn).unwrap();

        // 親タスク1つが取得される
        assert_eq!(hierarchy.len(), 1);
        assert_eq!(hierarchy[0].title, "Active Parent");

        // 子タスクは Draft のみ（Archived は除外）
        assert_eq!(hierarchy[0].children.len(), 1);
        assert_eq!(hierarchy[0].children[0].title, "Draft Child");
        assert_eq!(hierarchy[0].children[0].status, TaskStatus::Draft);
    }

    #[test]
    fn test_update_task_rejects_non_draft() {
        let mut conn = setup_test_db();

        // Draftタスクを作成
        let create_req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, create_req).unwrap();

        // ActiveステータスにするためにDieselを直接使用（Draft checkをバイパス）
        diesel::update(tasks::table.find(&task.id))
            .set(tasks::status.eq("active"))
            .execute(&mut conn)
            .unwrap();

        // Activeタスクの編集を試みる
        let update_req = UpdateTaskRequestInput {
            title: Some("Updated Title".to_string()),
            description: None,
            status: None,
            parent_id: None,
            tags: None,
        };

        let result = TaskService::update_task(&mut conn, &task.id, update_req);

        // Draft以外のタスクは編集不可（REQ-0016）
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::TaskNotDraft(_)));
    }

    #[test]
    fn test_delete_task_rejects_non_draft() {
        let mut conn = setup_test_db();

        // Draftタスクを作成
        let create_req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, create_req).unwrap();

        // CompletedステータスにするためにDieselを直接使用（Draft checkをバイパス）
        diesel::update(tasks::table.find(&task.id))
            .set(tasks::status.eq("completed"))
            .execute(&mut conn)
            .unwrap();

        // Completedタスクの削除を試みる
        let result = TaskService::delete_task(&mut conn, &task.id);

        // Draft以外のタスクは削除不可（REQ-0017）
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::TaskNotDraft(_)));
    }

    #[test]
    fn test_update_and_delete_draft_task_succeeds() {
        let mut conn = setup_test_db();

        // Draftタスクを作成
        let create_req = CreateTaskRequest {
            title: "Draft Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, create_req).unwrap();
        assert_eq!(task.status, TaskStatus::Draft);

        // Draftタスクの編集は成功する
        let update_req = UpdateTaskRequestInput {
            title: Some("Updated Draft Task".to_string()),
            description: None,
            status: None,
            parent_id: None,
            tags: None,
        };
        let updated_task = TaskService::update_task(&mut conn, &task.id, update_req).unwrap();
        assert_eq!(updated_task.title, "Updated Draft Task");

        // Draftタスクの削除は成功する
        let result = TaskService::delete_task(&mut conn, &task.id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_task_permanently_success() {
        let mut conn = setup_test_db();

        // Draftタスクを作成
        let create_req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, create_req).unwrap();

        // ArchivedステータスにするためにDieselを直接使用
        diesel::update(tasks::table.find(&task.id))
            .set(tasks::status.eq("archived"))
            .execute(&mut conn)
            .unwrap();

        // Archivedタスクの物理削除は成功する
        let result = TaskService::delete_task_permanently(&mut conn, &task.id);
        assert!(result.is_ok());

        // タスクが完全に削除されたことを確認
        let deleted_task = tasks::table
            .find(&task.id)
            .first::<Task>(&mut conn)
            .optional()
            .unwrap();
        assert!(deleted_task.is_none());
    }

    #[test]
    fn test_delete_task_permanently_rejects_non_archived() {
        let mut conn = setup_test_db();

        // Draftタスクを作成
        let create_req = CreateTaskRequest {
            title: "Draft Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, create_req).unwrap();

        // Draftタスクの物理削除は拒否される
        let result = TaskService::delete_task_permanently(&mut conn, &task.id);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::TaskNotArchived(_)
        ));

        // Activeタスクの物理削除も拒否される
        diesel::update(tasks::table.find(&task.id))
            .set(tasks::status.eq("active"))
            .execute(&mut conn)
            .unwrap();
        let result = TaskService::delete_task_permanently(&mut conn, &task.id);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::TaskNotArchived(_)
        ));

        // Completedタスクの物理削除も拒否される
        diesel::update(tasks::table.find(&task.id))
            .set(tasks::status.eq("completed"))
            .execute(&mut conn)
            .unwrap();
        let result = TaskService::delete_task_permanently(&mut conn, &task.id);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::TaskNotArchived(_)
        ));
    }

    #[test]
    fn test_delete_task_permanently_cascade_deletes_children() {
        let mut conn = setup_test_db();

        // 親タスクを作成
        let parent_req = CreateTaskRequest {
            title: "Parent Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

        // 子タスク1を作成
        let child1_req = CreateTaskRequest {
            title: "Child Task 1".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let child1 = TaskService::create_task(&mut conn, child1_req).unwrap();

        // 子タスク2を作成
        let child2_req = CreateTaskRequest {
            title: "Child Task 2".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        };
        let child2 = TaskService::create_task(&mut conn, child2_req).unwrap();

        // 親と子をすべてArchivedにする
        diesel::update(tasks::table.find(&parent.id))
            .set(tasks::status.eq("archived"))
            .execute(&mut conn)
            .unwrap();
        diesel::update(tasks::table.find(&child1.id))
            .set(tasks::status.eq("archived"))
            .execute(&mut conn)
            .unwrap();
        diesel::update(tasks::table.find(&child2.id))
            .set(tasks::status.eq("archived"))
            .execute(&mut conn)
            .unwrap();

        // 親タスクを物理削除
        let result = TaskService::delete_task_permanently(&mut conn, &parent.id);
        assert!(result.is_ok());

        // 親タスクが削除されたことを確認
        let deleted_parent = tasks::table
            .find(&parent.id)
            .first::<Task>(&mut conn)
            .optional()
            .unwrap();
        assert!(deleted_parent.is_none());

        // 子タスクもCASCADEで削除されたことを確認
        let deleted_child1 = tasks::table
            .find(&child1.id)
            .first::<Task>(&mut conn)
            .optional()
            .unwrap();
        assert!(deleted_child1.is_none());

        let deleted_child2 = tasks::table
            .find(&child2.id)
            .first::<Task>(&mut conn)
            .optional()
            .unwrap();
        assert!(deleted_child2.is_none());
    }

    #[test]
    fn test_delete_task_permanently_not_found() {
        let mut conn = setup_test_db();

        // 存在しないタスクIDで物理削除を試みる
        let result = TaskService::delete_task_permanently(&mut conn, "non-existent-id");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::TaskNotFound(_)));
    }

    // ==================== restore_task Tests ====================

    #[test]
    fn test_restore_task_success() {
        let mut conn = setup_test_db();

        // Draftタスクを作成
        let create_req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, create_req).unwrap();

        // ArchivedステータスにするためにDieselを直接使用
        diesel::update(tasks::table.find(&task.id))
            .set(tasks::status.eq("archived"))
            .execute(&mut conn)
            .unwrap();

        // Archivedタスクの復元は成功する
        let result = TaskService::restore_task(&mut conn, &task.id);
        assert!(result.is_ok());

        // タスクがDraft状態に戻っていることを確認
        let restored_task = result.unwrap();
        assert_eq!(restored_task.status, TaskStatus::Draft);
        assert_eq!(restored_task.id, task.id);
        assert_eq!(restored_task.title, task.title);

        // updated_atが更新されていることを確認
        assert!(restored_task.updated_at > task.updated_at);
    }

    #[test]
    fn test_restore_task_rejects_non_archived() {
        let mut conn = setup_test_db();

        // Draftタスクを作成
        let create_req = CreateTaskRequest {
            title: "Draft Task".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let draft_task = TaskService::create_task(&mut conn, create_req).unwrap();

        // Draftタスクの復元は拒否される
        let result = TaskService::restore_task(&mut conn, &draft_task.id);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::TaskNotArchived(_)
        ));

        // Activeタスクを作成
        diesel::update(tasks::table.find(&draft_task.id))
            .set(tasks::status.eq("active"))
            .execute(&mut conn)
            .unwrap();

        // Activeタスクの復元は拒否される
        let result = TaskService::restore_task(&mut conn, &draft_task.id);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::TaskNotArchived(_)
        ));

        // Completedタスクを作成
        diesel::update(tasks::table.find(&draft_task.id))
            .set(tasks::status.eq("completed"))
            .execute(&mut conn)
            .unwrap();

        // Completedタスクの復元は拒否される
        let result = TaskService::restore_task(&mut conn, &draft_task.id);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::TaskNotArchived(_)
        ));
    }

    #[test]
    fn test_restore_task_not_found() {
        let mut conn = setup_test_db();

        // 存在しないタスクIDで復元を試みる
        let result = TaskService::restore_task(&mut conn, "non-existent-id");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::TaskNotFound(_)));
    }

    // ==================== list_tasks status filtering Tests ====================

    #[test]
    fn test_list_tasks_default_draft_and_active() {
        let mut conn = setup_test_db();

        // 異なるステータスのタスクを作成
        let draft_task = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Draft Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        let active_task_id = {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: "Active Task".to_string(),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("active"))
                .execute(&mut conn)
                .unwrap();
            task.id
        };

        let completed_task_id = {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: "Completed Task".to_string(),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("completed"))
                .execute(&mut conn)
                .unwrap();
            task.id
        };

        let archived_task_id = {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: "Archived Task".to_string(),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("archived"))
                .execute(&mut conn)
                .unwrap();
            task.id
        };

        // status = None でデフォルト動作（Draft + Active）
        let result = TaskService::list_tasks(&mut conn, None).unwrap();

        assert_eq!(result.len(), 2);
        let task_ids: Vec<String> = result.iter().map(|t| t.id.clone()).collect();
        assert!(task_ids.contains(&draft_task.id));
        assert!(task_ids.contains(&active_task_id));
        assert!(!task_ids.contains(&completed_task_id));
        assert!(!task_ids.contains(&archived_task_id));
    }

    #[test]
    fn test_list_tasks_status_completed_only() {
        let mut conn = setup_test_db();

        // Completedタスクを作成
        let completed_task_id = {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: "Completed Task".to_string(),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("completed"))
                .execute(&mut conn)
                .unwrap();
            task.id
        };

        // Draftタスクを作成
        let draft_task = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Draft Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // status = Some(["completed"]) でCompletedのみ取得
        let result = TaskService::list_tasks(&mut conn, Some(vec!["completed".to_string()]))
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, completed_task_id);
        assert_eq!(result[0].status, TaskStatus::Completed);
    }

    #[test]
    fn test_list_tasks_status_archived_only() {
        let mut conn = setup_test_db();

        // Archivedタスクを作成
        let archived_task_id = {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: "Archived Task".to_string(),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("archived"))
                .execute(&mut conn)
                .unwrap();
            task.id
        };

        // Draftタスクを作成
        let _draft_task = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Draft Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // status = Some(["archived"]) でArchivedのみ取得
        let result =
            TaskService::list_tasks(&mut conn, Some(vec!["archived".to_string()])).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, archived_task_id);
        assert_eq!(result[0].status, TaskStatus::Archived);
    }

    #[test]
    fn test_list_tasks_status_multiple() {
        let mut conn = setup_test_db();

        // Draft, Active, Completedタスクを作成
        let draft_task = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Draft Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        let active_task_id = {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: "Active Task".to_string(),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("active"))
                .execute(&mut conn)
                .unwrap();
            task.id
        };

        let _completed_task = {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: "Completed Task".to_string(),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("completed"))
                .execute(&mut conn)
                .unwrap();
            task
        };

        // status = Some(["draft", "active"]) でDraft + Active取得
        let result = TaskService::list_tasks(
            &mut conn,
            Some(vec!["draft".to_string(), "active".to_string()]),
        )
        .unwrap();

        assert_eq!(result.len(), 2);
        let task_ids: Vec<String> = result.iter().map(|t| t.id.clone()).collect();
        assert!(task_ids.contains(&draft_task.id));
        assert!(task_ids.contains(&active_task_id));
    }

    #[test]
    fn test_list_tasks_status_empty_array() {
        let mut conn = setup_test_db();

        // タスクを作成
        let _task = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Test Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // status = Some([]) で全タスク取得（空配列は条件なし）
        let result = TaskService::list_tasks(&mut conn, Some(vec![])).unwrap();

        // 空配列の場合、eq_anyは何も返さないため結果も空
        // これは仕様として正しい（フィルタ条件が空なので）
        assert!(result.is_empty());
    }

    // ===== ページネーション機能のテスト (REQ-0024) =====

    #[test]
    fn test_list_tasks_paginated_default_values() {
        let mut conn = setup_test_db();

        // 25個のタスクを作成（Draft状態）
        for i in 1..=25 {
            TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: format!("Task {}", i),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
        }

        // デフォルト値（limit=20, offset=0）でページネーション
        let params = ListTasksPaginatedParams::default();
        let result = TaskService::list_tasks_paginated(&mut conn, params).unwrap();

        // 検証
        assert_eq!(result.tasks.len(), 20, "デフォルトlimit=20で20件取得");
        assert_eq!(result.total, 25, "総件数は25件");
    }

    #[test]
    fn test_list_tasks_paginated_with_offset() {
        let mut conn = setup_test_db();

        // 30個のタスクを作成
        for i in 1..=30 {
            TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: format!("Task {}", i),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
        }

        // 1ページ目（offset=0, limit=10）
        let page1 = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams {
                status: None,
                limit: Some(10),
                offset: Some(0),
            },
        )
        .unwrap();

        // 2ページ目（offset=10, limit=10）
        let page2 = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams {
                status: None,
                limit: Some(10),
                offset: Some(10),
            },
        )
        .unwrap();

        // 検証
        assert_eq!(page1.tasks.len(), 10, "1ページ目は10件");
        assert_eq!(page1.total, 30, "総件数は30件");
        assert_eq!(page2.tasks.len(), 10, "2ページ目は10件");
        assert_eq!(page2.total, 30, "総件数は30件");

        // ページ間でタスクが重複していないことを確認
        let page1_ids: Vec<String> = page1.tasks.iter().map(|t| t.id.clone()).collect();
        let page2_ids: Vec<String> = page2.tasks.iter().map(|t| t.id.clone()).collect();
        for id in &page1_ids {
            assert!(
                !page2_ids.contains(id),
                "ページ間でタスクが重複していない"
            );
        }
    }

    #[test]
    fn test_list_tasks_paginated_total_count_accuracy() {
        let mut conn = setup_test_db();

        // Draft: 10個、Active: 5個、Completed: 3個
        for i in 1..=10 {
            TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: format!("Draft Task {}", i),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
        }

        for i in 1..=5 {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: format!("Active Task {}", i),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            // Draft -> Active（diesel::updateで直接設定）
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("active"))
                .execute(&mut conn)
                .unwrap();
        }

        for i in 1..=3 {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: format!("Completed Task {}", i),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            // Draft -> Completed（diesel::updateで直接設定）
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("completed"))
                .execute(&mut conn)
                .unwrap();
        }

        // デフォルト（Draft + Active）の総件数確認
        let result_default = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams::default(),
        )
        .unwrap();
        assert_eq!(result_default.total, 15, "Draft + Active = 15件");

        // Completedのみの総件数確認
        let result_completed = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams {
                status: Some(vec!["completed".to_string()]),
                limit: None,
                offset: None,
            },
        )
        .unwrap();
        assert_eq!(result_completed.total, 3, "Completed = 3件");
        assert_eq!(result_completed.tasks.len(), 3, "Completed = 3件取得");
    }

    #[test]
    fn test_list_tasks_paginated_status_filter() {
        let mut conn = setup_test_db();

        // Draft: 2個、Completed: 3個
        for i in 1..=2 {
            TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: format!("Draft Task {}", i),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
        }

        for i in 1..=3 {
            let task = TaskService::create_task(
                &mut conn,
                CreateTaskRequest {
                    title: format!("Task {}", i),
                    description: None,
                    tags: vec![],
                    parent_id: None,
                },
            )
            .unwrap();
            // Draft -> Completed（diesel::updateで直接設定）
            diesel::update(tasks::table.find(&task.id))
                .set(tasks::status.eq("completed"))
                .execute(&mut conn)
                .unwrap();
        }

        // statusフィルタ: Completedのみ
        let result = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams {
                status: Some(vec!["completed".to_string()]),
                limit: Some(10),
                offset: Some(0),
            },
        )
        .unwrap();

        assert_eq!(result.total, 3, "Completedは3件");
        assert_eq!(result.tasks.len(), 3, "Completedを3件取得");
        for task in &result.tasks {
            assert_eq!(
                task.status,
                TaskStatus::Completed,
                "全てCompletedステータス"
            );
        }

        // statusフィルタ: 空配列（結果なし）
        let result_empty = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams {
                status: Some(vec![]),
                limit: None,
                offset: None,
            },
        )
        .unwrap();

        assert_eq!(result_empty.total, 0, "空配列の場合、総件数0");
        assert!(result_empty.tasks.is_empty(), "空配列の場合、結果なし");
    }

    #[test]
    fn test_list_tasks_paginated_parent_title_enrichment() {
        let mut conn = setup_test_db();

        // 親タスクを作成
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

        // 子タスクを作成
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

        // 子タスクをCompletedに変更
        diesel::update(tasks::table.find(&child.id))
            .set(tasks::status.eq("completed"))
            .execute(&mut conn)
            .unwrap();

        // list_tasks_paginatedでCompletedタスクを取得
        let result = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams {
                status: Some(vec!["completed".to_string()]),
                limit: Some(10),
                offset: Some(0),
            },
        )
        .unwrap();

        assert_eq!(result.tasks.len(), 1, "Completedタスクは1件");
        let returned_child = &result.tasks[0];

        // parent_titleが正しく設定されていることを確認
        assert_eq!(returned_child.parent_title, Some("Parent Task".to_string()));
        assert_eq!(returned_child.parent_id, Some(parent.id.clone()));
    }

    #[test]
    fn test_list_tasks_paginated_root_task_no_parent_title() {
        let mut conn = setup_test_db();

        // ルートタスク（親なし）を作成
        let root = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Root Task".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // ルートタスクをCompletedに変更
        diesel::update(tasks::table.find(&root.id))
            .set(tasks::status.eq("completed"))
            .execute(&mut conn)
            .unwrap();

        // list_tasks_paginatedでCompletedタスクを取得
        let result = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams {
                status: Some(vec!["completed".to_string()]),
                limit: Some(10),
                offset: Some(0),
            },
        )
        .unwrap();

        assert_eq!(result.tasks.len(), 1, "Completedタスクは1件");
        let returned_root = &result.tasks[0];

        // ルートタスクはparent_titleがNoneであることを確認
        assert_eq!(returned_root.parent_title, None);
        assert_eq!(returned_root.parent_id, None);
    }

    #[test]
    fn test_list_tasks_paginated_multiple_children_batch_fetch() {
        let mut conn = setup_test_db();

        // 親タスクを2つ作成
        let parent1 = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Parent 1".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        let parent2 = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Parent 2".to_string(),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();

        // 各親に子タスクを作成
        let child1 = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Child 1".to_string(),
                description: None,
                tags: vec![],
                parent_id: Some(parent1.id.clone()),
            },
        )
        .unwrap();

        let child2 = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: "Child 2".to_string(),
                description: None,
                tags: vec![],
                parent_id: Some(parent2.id.clone()),
            },
        )
        .unwrap();

        // 子タスクをCompletedに変更
        diesel::update(tasks::table.find(&child1.id))
            .set(tasks::status.eq("completed"))
            .execute(&mut conn)
            .unwrap();

        diesel::update(tasks::table.find(&child2.id))
            .set(tasks::status.eq("completed"))
            .execute(&mut conn)
            .unwrap();

        // list_tasks_paginatedでCompletedタスクを取得（バッチフェッチのテスト）
        let result = TaskService::list_tasks_paginated(
            &mut conn,
            ListTasksPaginatedParams {
                status: Some(vec!["completed".to_string()]),
                limit: Some(10),
                offset: Some(0),
            },
        )
        .unwrap();

        assert_eq!(result.tasks.len(), 2, "Completedタスクは2件");

        // 各子タスクが正しい親タイトルを持つことを確認
        for task in &result.tasks {
            if task.id == child1.id {
                assert_eq!(task.parent_title, Some("Parent 1".to_string()));
            } else if task.id == child2.id {
                assert_eq!(task.parent_title, Some("Parent 2".to_string()));
            } else {
                panic!("Unexpected task ID");
            }
        }
    }

    #[test]
    fn test_parent_updated_at_changes_when_child_status_changes() {
        let mut conn = setup_test_db();

        // 親タスク作成
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

        // 親タスクの初期updated_atを記録
        let initial_updated_at = parent.updated_at.clone();

        // 少し待機（タイムスタンプの差を明確にするため）
        std::thread::sleep(std::time::Duration::from_millis(10));

        // 子タスク作成
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

        // 子タスクをActiveに変更（親タスクのステータスも自動的にActiveになる）
        let update_req = UpdateTaskRequestInput {
            title: None,
            description: None,
            status: Some("active".to_string()),
            parent_id: None,
            tags: None,
        };
        TaskService::update_task(&mut conn, &child.id, update_req).unwrap();

        // 親タスクを再取得
        let updated_parent = TaskService::get_task(&mut conn, &parent.id).unwrap();

        // 親タスクのステータスがActiveに変更されていることを確認
        assert_eq!(updated_parent.status, TaskStatus::Active);

        // 親タスクのupdated_atが更新されていることを確認
        assert_ne!(
            updated_parent.updated_at, initial_updated_at,
            "親タスクのupdated_atが更新されていない"
        );

        // updated_atが初期値より後であることを確認
        assert!(
            updated_parent.updated_at > initial_updated_at,
            "updated_atが初期値より古い: {} <= {}",
            updated_parent.updated_at,
            initial_updated_at
        );
    }
}
