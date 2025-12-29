use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use tms_v2_lib::models::tag::{CreateTagRequest, UpdateTagRequest};
use tms_v2_lib::models::task::{CreateTaskRequest, SearchTasksParams, TaskStatus, UpdateTaskRequest};
use tms_v2_lib::schema::tasks;
use tms_v2_lib::service::{QueueService, TagService, TaskService};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// テスト用のDBプールを作成
fn setup_test_pool() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // マイグレーション実行
    let mut conn = pool.get().expect("Failed to get connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    pool
}

// ========================================
// Task API Tests (8 tests)
// ========================================

#[test]
fn test_create_task_success() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    let req = CreateTaskRequest {
        title: "統合テスト用タスク".to_string(),
        description: Some("説明文".to_string()),
        tags: vec![],
        parent_id: None,
    };

    let result = TaskService::create_task(&mut conn, req);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.title, "統合テスト用タスク");
    assert_eq!(task.status, TaskStatus::Draft);
}

#[test]
fn test_create_task_invalid_input() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    let req = CreateTaskRequest {
        title: "   ".to_string(), // 空のタイトル
        description: None,
        tags: vec![],
        parent_id: None,
    };

    let result = TaskService::create_task(&mut conn, req);
    assert!(result.is_err());
}

#[test]
fn test_get_task_success() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成
    let req = CreateTaskRequest {
        title: "取得テスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let created = TaskService::create_task(&mut conn, req).unwrap();

    // タスクを取得
    let result = TaskService::get_task(&mut conn, &created.id);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, created.id);
    assert_eq!(task.title, "取得テスト");
}

#[test]
fn test_get_task_not_found() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    let result = TaskService::get_task(&mut conn, "nonexistent-id");
    assert!(result.is_err());
}

#[test]
fn test_update_task_success() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成
    let req = CreateTaskRequest {
        title: "更新前".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let created = TaskService::create_task(&mut conn, req).unwrap();

    // タスクを更新
    let update_req = UpdateTaskRequest {
        title: Some("更新後".to_string()),
        description: Some("新しい説明".to_string()),
        parent_id: None,
        status: None,
        updated_at: None,
    };
    let result = TaskService::update_task(&mut conn, &created.id, update_req);
    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.title, "更新後");
}

#[test]
fn test_delete_task_success() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成
    let req = CreateTaskRequest {
        title: "削除テスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let created = TaskService::create_task(&mut conn, req).unwrap();

    // タスクを削除
    let result = TaskService::delete_task(&mut conn, &created.id);
    assert!(result.is_ok());

    // 削除されたことを確認（論理削除なのでステータスがArchivedに変更される）
    let deleted_task = TaskService::get_task(&mut conn, &created.id).unwrap();
    assert_eq!(deleted_task.status, TaskStatus::Archived);
}

#[test]
fn test_list_tasks() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 複数のタスクを作成
    for i in 1..=3 {
        let req = CreateTaskRequest {
            title: format!("タスク{}", i),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        TaskService::create_task(&mut conn, req).unwrap();
    }

    // タスクリストを取得
    let result = TaskService::list_tasks(&mut conn, None);
    assert!(result.is_ok());
    let tasks = result.unwrap();
    assert_eq!(tasks.len(), 3);
}

#[test]
fn test_search_tasks_with_filters() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // テストタスクを作成
    let req1 = CreateTaskRequest {
        title: "検索対象1".to_string(),
        description: Some("キーワード含む".to_string()),
        tags: vec![],
        parent_id: None,
    };
    TaskService::create_task(&mut conn, req1).unwrap();

    let req2 = CreateTaskRequest {
        title: "検索対象2".to_string(),
        description: Some("別の説明".to_string()),
        tags: vec![],
        parent_id: None,
    };
    TaskService::create_task(&mut conn, req2).unwrap();

    // キーワード検索
    let params = SearchTasksParams {
        q: Some("キーワード".to_string()),
        status: None,
        tags: None,
    };
    let result = TaskService::search_tasks(&mut conn, params);
    assert!(result.is_ok());
    let tasks = result.unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "検索対象1");
}

// ========================================
// Queue API Tests (8 tests)
// ========================================

#[test]
fn test_add_to_queue_success() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成
    let req = CreateTaskRequest {
        title: "キューテスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();

    // キューに追加（自動的にActiveに変更される）
    let result = QueueService::add_to_queue(&mut conn, task.id.clone());
    assert!(result.is_ok());
}

#[test]
fn test_add_to_queue_duplicate() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成
    let req = CreateTaskRequest {
        title: "重複テスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();

    // 1回目の追加
    QueueService::add_to_queue(&mut conn, task.id.clone()).unwrap();

    // 2回目の追加（重複エラー）
    let result = QueueService::add_to_queue(&mut conn, task.id.clone());
    assert!(result.is_err());
}

#[test]
fn test_get_queue() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成してキューに追加
    for i in 1..=2 {
        let req = CreateTaskRequest {
            title: format!("キュータスク{}", i),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, req).unwrap();
        QueueService::add_to_queue(&mut conn, task.id).unwrap();
    }

    // キューを取得
    let result = QueueService::get_queue(&mut conn);
    assert!(result.is_ok());
    let queue = result.unwrap();
    assert_eq!(queue.len(), 2);
}

#[test]
fn test_remove_from_queue() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成してキューに追加
    let req = CreateTaskRequest {
        title: "削除テスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();
    let task_id = task.id.clone();

    QueueService::add_to_queue(&mut conn, task_id.clone()).unwrap();

    // キューから削除（draftに戻す）
    let result = QueueService::remove_from_queue(&mut conn, task_id, "draft".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_clear_queue() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 複数タスクをキューに追加
    for i in 1..=3 {
        let req = CreateTaskRequest {
            title: format!("クリアテスト{}", i),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, req).unwrap();
        QueueService::add_to_queue(&mut conn, task.id).unwrap();
    }

    // キューをクリア
    let result = QueueService::clear_queue(&mut conn);
    assert!(result.is_ok());

    // 空になったことを確認
    let queue = QueueService::get_queue(&mut conn).unwrap();
    assert_eq!(queue.len(), 0);
}

#[test]
fn test_update_queue_position() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成してキューに追加
    let req = CreateTaskRequest {
        title: "位置更新テスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();
    let task_id = task.id.clone();

    QueueService::add_to_queue(&mut conn, task_id.clone()).unwrap();

    // 位置を更新
    let result = QueueService::update_queue_position(&mut conn, task_id, 0);
    assert!(result.is_ok());
}

#[test]
fn test_reorder_queue() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 複数タスクを作成してキューに追加
    let mut task_ids = vec![];
    for i in 1..=3 {
        let req = CreateTaskRequest {
            title: format!("並び替えテスト{}", i),
            description: None,
            tags: vec![],
            parent_id: None,
        };
        let task = TaskService::create_task(&mut conn, req).unwrap();
        let task_id = task.id.clone();
        QueueService::add_to_queue(&mut conn, task_id.clone()).unwrap();
        task_ids.push(task_id);
    }

    // 並び替え（逆順）
    task_ids.reverse();
    let result = QueueService::reorder_queue(&mut conn, task_ids.clone());
    assert!(result.is_ok());

    // 順序を確認
    let queue = QueueService::get_queue(&mut conn).unwrap();
    assert_eq!(queue[0].task_id, task_ids[0]);
    assert_eq!(queue[1].task_id, task_ids[1]);
    assert_eq!(queue[2].task_id, task_ids[2]);
}

#[test]
fn test_reorder_queue_invalid_size() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを1つだけキューに追加
    let req = CreateTaskRequest {
        title: "不正な並び替え".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();
    QueueService::add_to_queue(&mut conn, task.id).unwrap();

    // サイズが一致しない並び替え要求
    let result = QueueService::reorder_queue(&mut conn, vec![]); // 空の配列
    assert!(result.is_err());
}

// ========================================
// Tag API Tests (6 tests)
// ========================================

#[test]
fn test_create_tag_success() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    let req = CreateTagRequest {
        name: "テストタグ".to_string(),
        color: None,
    };

    let result = TagService::create_tag(&mut conn, req);
    assert!(result.is_ok());
    let tag = result.unwrap();
    assert_eq!(tag.name, "テストタグ");
    assert_eq!(tag.usage_count, 0);
}

#[test]
fn test_create_tag_empty_name() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    let req = CreateTagRequest {
        name: "   ".to_string(), // 空の名前
        color: None,
    };

    let result = TagService::create_tag(&mut conn, req);
    assert!(result.is_err());
}

#[test]
fn test_list_tags() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 複数のタグを作成
    for i in 1..=3 {
        let req = CreateTagRequest {
            name: format!("タグ{}", i),
            color: None,
        };
        TagService::create_tag(&mut conn, req).unwrap();
    }

    // タグリストを取得
    let result = TagService::list_tags(&mut conn);
    assert!(result.is_ok());
    let tags = result.unwrap();
    assert_eq!(tags.len(), 3);
}

#[test]
fn test_update_tag() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タグを作成
    let req = CreateTagRequest {
        name: "更新前".to_string(),
        color: None,
    };
    let created = TagService::create_tag(&mut conn, req).unwrap();

    // タグを更新
    let update_req = UpdateTagRequest {
        name: Some("更新後".to_string()),
        color: None,
        updated_at: None,
    };
    let result = TagService::update_tag(&mut conn, &created.id, update_req);
    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.name, "更新後");
}

#[test]
fn test_delete_tag_success() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タグを作成
    let req = CreateTagRequest {
        name: "削除テスト".to_string(),
        color: None,
    };
    let created = TagService::create_tag(&mut conn, req).unwrap();

    // タグを削除
    let result = TagService::delete_tag(&mut conn, &created.id);
    assert!(result.is_ok());
}

#[test]
fn test_delete_tag_in_use() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タグを作成
    let tag_req = CreateTagRequest {
        name: "使用中タグ".to_string(),
        color: None,
    };
    let tag = TagService::create_tag(&mut conn, tag_req).unwrap();

    // タグ付きタスクを作成
    let task_req = CreateTaskRequest {
        title: "タグ付きタスク".to_string(),
        description: None,
        tags: vec![tag.name.clone()],
        parent_id: None,
    };
    TaskService::create_task(&mut conn, task_req).unwrap();

    // 使用中のタグを削除しようとする
    let result = TagService::delete_tag(&mut conn, &tag.id);
    assert!(result.is_err());
}

// ========================================
// Integration Scenario Tests (3 tests)
// ========================================

#[test]
fn test_complete_workflow() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 1. タグを作成
    let tag_req = CreateTagRequest {
        name: "重要".to_string(),
        color: None,
    };
    let tag = TagService::create_tag(&mut conn, tag_req).unwrap();

    // 2. タスクを作成
    let task_req = CreateTaskRequest {
        title: "統合テストタスク".to_string(),
        description: Some("完全なワークフローテスト".to_string()),
        tags: vec![tag.name.clone()],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, task_req).unwrap();

    // 3. キューに追加（自動的にActiveに変更される）
    QueueService::add_to_queue(&mut conn, task.id.clone()).unwrap();

    // 4. キューを取得して確認
    let queue = QueueService::get_queue(&mut conn).unwrap();
    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0].task_id, task.id);

    // 5. タスクを完了に変更（Activeタスクのため、Dieselで直接更新）
    diesel::update(tasks::table.find(&task.id))
        .set(tasks::status.eq("completed"))
        .execute(&mut conn)
        .unwrap();

    // 6. タスクを取得して状態確認
    let final_task = TaskService::get_task(&mut conn, &task.id).unwrap();
    assert_eq!(final_task.status, TaskStatus::Completed);
}

#[test]
fn test_parent_child_task_workflow() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 1. 親タスクを作成
    let parent_req = CreateTaskRequest {
        title: "親タスク".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

    // 2. 子タスクを作成
    let child_req = CreateTaskRequest {
        title: "子タスク".to_string(),
        description: None,
        tags: vec![],
        parent_id: Some(parent.id.clone()),
    };
    let child = TaskService::create_task(&mut conn, child_req).unwrap();
    assert_eq!(child.parent_id, Some(parent.id.clone()));

    // 3. 親タスクを取得して子タスク情報が含まれることを確認
    let retrieved_parent = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert!(retrieved_parent.children_ids.len() > 0);
    assert!(retrieved_parent.children_ids.contains(&child.id));

    // 4. 親タスクを削除しようとする（失敗するはず：子タスクが存在）
    let delete_result = TaskService::delete_task(&mut conn, &parent.id);
    assert!(delete_result.is_err());

    // 5. 孫タスクの作成を試みる（失敗するはず：BR-016により2レベル制限）
    let grandchild_req = CreateTaskRequest {
        title: "孫タスク".to_string(),
        description: None,
        tags: vec![],
        parent_id: Some(child.id.clone()),
    };
    let grandchild_result = TaskService::create_task(&mut conn, grandchild_req);
    assert!(grandchild_result.is_err()); // 孫タスク作成は禁止されている
}

#[test]
fn test_tag_filter_workflow() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 1. 複数のタグを作成
    let tag1 = TagService::create_tag(
        &mut conn,
        CreateTagRequest {
            name: "バグ".to_string(),
            color: None,
        },
    )
    .unwrap();

    let tag2 = TagService::create_tag(
        &mut conn,
        CreateTagRequest {
            name: "機能".to_string(),
            color: None,
        },
    )
    .unwrap();

    // 2. 異なるタグを持つタスクを作成
    TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "バグ修正".to_string(),
            description: None,
            tags: vec![tag1.name.clone()],
            parent_id: None,
        },
    )
    .unwrap();

    TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "新機能".to_string(),
            description: None,
            tags: vec![tag2.name.clone()],
            parent_id: None,
        },
    )
    .unwrap();

    // 3. タグでフィルタ検索
    let params = SearchTasksParams {
        q: None,
        status: None,
        tags: Some(vec![tag1.name]),
    };
    let results = TaskService::search_tasks(&mut conn, params).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "バグ修正");
}

// ========================================
// Parent-Child Status Sync Tests (2 tests)
// ========================================

#[test]
fn test_parent_status_sync_on_child_update() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 親タスク作成
    let parent_req = CreateTaskRequest {
        title: "親タスク".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

    // 子タスク1作成（Draft）
    let child1_req = CreateTaskRequest {
        title: "子タスク1".to_string(),
        description: None,
        tags: vec![],
        parent_id: Some(parent.id.clone()),
    };
    let child1 = TaskService::create_task(&mut conn, child1_req).unwrap();

    // 子タスク2作成（Draft）
    let child2_req = CreateTaskRequest {
        title: "子タスク2".to_string(),
        description: None,
        tags: vec![],
        parent_id: Some(parent.id.clone()),
    };
    TaskService::create_task(&mut conn, child2_req).unwrap();

    // 親タスクのステータスを確認（全子がDraft → 親もDraft）
    let parent_task = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_task.status, TaskStatus::Draft);

    // 子タスク1をActiveに変更
    let update_req = UpdateTaskRequest {
        title: None,
        description: None,
        status: Some("active".to_string()),
        parent_id: None,
        updated_at: None,
    };
    TaskService::update_task(&mut conn, &child1.id, update_req).unwrap();

    // 親タスクのステータスを確認（1つでもActive → 親もActive）
    let parent_task = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_task.status, TaskStatus::Active);
}

#[test]
fn test_parent_status_sync_on_child_delete() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 親タスク作成
    let parent_req = CreateTaskRequest {
        title: "親タスク".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

    // 子タスク1作成（Draft）
    let child1_req = CreateTaskRequest {
        title: "子タスク1".to_string(),
        description: None,
        tags: vec![],
        parent_id: Some(parent.id.clone()),
    };
    let child1 = TaskService::create_task(&mut conn, child1_req).unwrap();

    // 子タスク1をActiveに変更
    let update_req = UpdateTaskRequest {
        title: None,
        description: None,
        status: Some("active".to_string()),
        parent_id: None,
        updated_at: None,
    };
    TaskService::update_task(&mut conn, &child1.id, update_req).unwrap();

    // 親タスクのステータスを確認（子がActive → 親もActive）
    let parent_task = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_task.status, TaskStatus::Active);

    // 子タスク1をDraftに戻してから削除（REQ-0017: Draft のみ削除可能）
    diesel::update(tasks::table.find(&child1.id))
        .set(tasks::status.eq("draft"))
        .execute(&mut conn)
        .unwrap();
    TaskService::delete_task(&mut conn, &child1.id).unwrap();

    // 親タスクのステータスを確認（全子がArchived → 親はCompleted）
    let parent_task = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_task.status, TaskStatus::Completed);
}

#[test]
fn test_queue_registration_restriction() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 親タスク作成
    let parent_req = CreateTaskRequest {
        title: "親タスク".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let parent = TaskService::create_task(&mut conn, parent_req).unwrap();

    // 子タスク作成
    let child_req = CreateTaskRequest {
        title: "子タスク".to_string(),
        description: None,
        tags: vec![],
        parent_id: Some(parent.id.clone()),
    };
    let child = TaskService::create_task(&mut conn, child_req).unwrap();

    // 親タスクをキューに追加しようとする（失敗するはず：BR-015）
    let parent_result = QueueService::add_to_queue(&mut conn, parent.id.clone());
    assert!(parent_result.is_err());

    // 子タスクをキューに追加する（成功するはず）
    let child_result = QueueService::add_to_queue(&mut conn, child.id.clone());
    assert!(child_result.is_ok());

    // キューを確認（子タスクのみ）
    let queue = QueueService::get_queue(&mut conn).unwrap();
    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0].task_title, "子タスク");
}

// ===== QueueService 親ステータス更新テスト =====

#[test]
fn test_queue_add_updates_parent_status() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 親タスク作成（Draft）
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

    // 子タスク作成（Draft）
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

    // 親タスクは Draft であることを確認
    let parent_before = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_before.status, TaskStatus::Draft);

    // 子タスクをキューに追加
    QueueService::add_to_queue(&mut conn, child.id.clone()).unwrap();

    // 子タスクが Active になることを確認
    let child_after = TaskService::get_task(&mut conn, &child.id).unwrap();
    assert_eq!(child_after.status, TaskStatus::Active);

    // 親タスクも Active になることを確認
    let parent_after = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_after.status, TaskStatus::Active);
}

#[test]
fn test_queue_remove_updates_parent_status() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

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

    // 子タスク1作成
    let child1 = TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "子タスク1".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        },
    )
    .unwrap();

    // 子タスク2作成
    let child2 = TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "子タスク2".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        },
    )
    .unwrap();

    // 両方の子タスクをキューに追加
    QueueService::add_to_queue(&mut conn, child1.id.clone()).unwrap();
    QueueService::add_to_queue(&mut conn, child2.id.clone()).unwrap();

    // 親タスクは Active になっているはず
    let parent_mid = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_mid.status, TaskStatus::Active);

    // 子タスク1をキューから削除（draft に戻す）
    QueueService::remove_from_queue(&mut conn, child1.id.clone(), "draft".to_string()).unwrap();

    // 親タスクはまだ Active のまま（子タスク2が Active のため）
    let parent_mid2 = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_mid2.status, TaskStatus::Active);

    // 子タスク2もキューから削除（draft に戻す）
    QueueService::remove_from_queue(&mut conn, child2.id.clone(), "draft".to_string()).unwrap();

    // 親タスクも Draft に戻ることを確認
    let parent_after = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_after.status, TaskStatus::Draft);
}

#[test]
fn test_queue_clear_updates_parent_status() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

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

    // 子タスクをキューに追加
    QueueService::add_to_queue(&mut conn, child.id.clone()).unwrap();

    // 親タスクは Active になっているはず
    let parent_mid = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_mid.status, TaskStatus::Active);

    // キュー全体をクリア
    QueueService::clear_queue(&mut conn).unwrap();

    // 子タスクが Draft に戻ることを確認
    let child_after = TaskService::get_task(&mut conn, &child.id).unwrap();
    assert_eq!(child_after.status, TaskStatus::Draft);

    // 親タスクも Draft に戻ることを確認
    let parent_after = TaskService::get_task(&mut conn, &parent.id).unwrap();
    assert_eq!(parent_after.status, TaskStatus::Draft);
}

#[test]
fn test_hierarchy_completed_children_workflow() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 親タスク作成（Draft）
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

    // 子タスク1作成（Draft）
    let child1 = TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "子タスク1".to_string(),
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
            title: "子タスク2".to_string(),
            description: None,
            tags: vec![],
            parent_id: Some(parent.id.clone()),
        },
    )
    .unwrap();

    // 子タスク1をキューに追加 → Active になる
    QueueService::add_to_queue(&mut conn, child1.id.clone()).unwrap();

    // 子タスク1を完了させる（Completed） - ActiveステータスのためDieselで直接更新（REQ-0016をバイパス）
    diesel::update(tasks::table.find(&child1.id))
        .set(tasks::status.eq("completed"))
        .execute(&mut conn)
        .unwrap();

    // get_hierarchy で取得
    let hierarchy = TaskService::get_hierarchy(&mut conn).unwrap();

    // 親タスクが1つ取得されることを確認
    assert_eq!(hierarchy.len(), 1);
    assert_eq!(hierarchy[0].id, parent.id);
    assert_eq!(hierarchy[0].title, "親タスク");

    // 子タスクが2つ含まれることを確認（Draft + Completed）
    assert_eq!(hierarchy[0].children.len(), 2);

    // 子タスクのIDを確認
    let child_ids: Vec<String> = hierarchy[0]
        .children
        .iter()
        .map(|c| c.id.clone())
        .collect();
    assert!(child_ids.contains(&child1.id));
    assert!(child_ids.contains(&child2.id));

    // Completed 子タスクが含まれていることを確認
    let completed_child = hierarchy[0]
        .children
        .iter()
        .find(|c| c.id == child1.id)
        .unwrap();
    assert_eq!(completed_child.status, TaskStatus::Completed);

    // Draft 子タスクも含まれていることを確認
    let draft_child = hierarchy[0]
        .children
        .iter()
        .find(|c| c.id == child2.id)
        .unwrap();
    assert_eq!(draft_child.status, TaskStatus::Draft);
}

// ========================================
// New Feature Integration Tests (5 tests)
// REQ-0016, REQ-0017, REQ-0018, REQ-0019, REQ-0022
// ========================================

#[test]
fn test_update_task_rejects_non_draft() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成（Draft）
    let req = CreateTaskRequest {
        title: "編集制限テスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();

    // Draft状態のタスクは更新可能
    let update_req = UpdateTaskRequest {
        title: Some("更新成功".to_string()),
        description: None,
        parent_id: None,
        status: None,
        updated_at: None,
    };
    let result = TaskService::update_task(&mut conn, &task.id, update_req);
    assert!(result.is_ok());

    // タスクをActiveに変更
    diesel::update(tasks::table.find(&task.id))
        .set(tasks::status.eq("active"))
        .execute(&mut conn)
        .unwrap();

    // Active状態のタスクは更新不可（REQ-0016）
    let update_req2 = UpdateTaskRequest {
        title: Some("更新失敗".to_string()),
        description: None,
        parent_id: None,
        status: None,
        updated_at: None,
    };
    let result2 = TaskService::update_task(&mut conn, &task.id, update_req2);
    assert!(result2.is_err());

    // タスクをCompletedに変更
    diesel::update(tasks::table.find(&task.id))
        .set(tasks::status.eq("completed"))
        .execute(&mut conn)
        .unwrap();

    // Completed状態のタスクも更新不可
    let update_req3 = UpdateTaskRequest {
        title: Some("更新失敗2".to_string()),
        description: None,
        parent_id: None,
        status: None,
        updated_at: None,
    };
    let result3 = TaskService::update_task(&mut conn, &task.id, update_req3);
    assert!(result3.is_err());
}

#[test]
fn test_delete_task_rejects_non_draft() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成（Draft）
    let req = CreateTaskRequest {
        title: "削除制限テスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();

    // タスクをActiveに変更
    diesel::update(tasks::table.find(&task.id))
        .set(tasks::status.eq("active"))
        .execute(&mut conn)
        .unwrap();

    // Active状態のタスクは削除不可（REQ-0017）
    let result = TaskService::delete_task(&mut conn, &task.id);
    assert!(result.is_err());

    // タスクをCompletedに変更
    diesel::update(tasks::table.find(&task.id))
        .set(tasks::status.eq("completed"))
        .execute(&mut conn)
        .unwrap();

    // Completed状態のタスクも削除不可
    let result2 = TaskService::delete_task(&mut conn, &task.id);
    assert!(result2.is_err());

    // タスクをDraftに戻す
    diesel::update(tasks::table.find(&task.id))
        .set(tasks::status.eq("draft"))
        .execute(&mut conn)
        .unwrap();

    // Draft状態のタスクは削除可能（論理削除 → Archived）
    let result3 = TaskService::delete_task(&mut conn, &task.id);
    assert!(result3.is_ok());

    // 削除されたことを確認（Archivedに変更）
    let deleted_task = TaskService::get_task(&mut conn, &task.id).unwrap();
    assert_eq!(deleted_task.status, TaskStatus::Archived);
}

#[test]
fn test_delete_task_permanently() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成（Draft）
    let req = CreateTaskRequest {
        title: "物理削除テスト".to_string(),
        description: None,
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();

    // Draft状態のタスクは物理削除不可
    let result = TaskService::delete_task_permanently(&mut conn, &task.id);
    assert!(result.is_err());

    // タスクをArchivedに変更
    diesel::update(tasks::table.find(&task.id))
        .set(tasks::status.eq("archived"))
        .execute(&mut conn)
        .unwrap();

    // Archived状態のタスクは物理削除可能（REQ-0018）
    let result2 = TaskService::delete_task_permanently(&mut conn, &task.id);
    assert!(result2.is_ok());

    // データベースから完全に削除されたことを確認
    let result3 = TaskService::get_task(&mut conn, &task.id);
    assert!(result3.is_err()); // TaskNotFound
}

#[test]
fn test_restore_task() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // タスクを作成（Draft）
    let req = CreateTaskRequest {
        title: "復元テスト".to_string(),
        description: Some("復元されるタスク".to_string()),
        tags: vec![],
        parent_id: None,
    };
    let task = TaskService::create_task(&mut conn, req).unwrap();

    // Draft状態のタスクはrestore不可
    let result = TaskService::restore_task(&mut conn, &task.id);
    assert!(result.is_err());

    // タスクをArchivedに変更
    diesel::update(tasks::table.find(&task.id))
        .set(tasks::status.eq("archived"))
        .execute(&mut conn)
        .unwrap();

    // Archived状態のタスクはrestore可能（REQ-0022）
    let result2 = TaskService::restore_task(&mut conn, &task.id);
    assert!(result2.is_ok());

    let restored_task = result2.unwrap();
    assert_eq!(restored_task.status, TaskStatus::Draft);
    assert_eq!(restored_task.title, "復元テスト");
    assert_eq!(restored_task.description, Some("復元されるタスク".to_string()));
}

#[test]
fn test_list_tasks_with_status_filter() {
    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 複数のタスクを異なるステータスで作成
    let draft_task = TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "Draft タスク".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        },
    )
    .unwrap();

    let active_task = TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "Active タスク".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        },
    )
    .unwrap();

    let completed_task = TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "Completed タスク".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        },
    )
    .unwrap();

    let archived_task = TaskService::create_task(
        &mut conn,
        CreateTaskRequest {
            title: "Archived タスク".to_string(),
            description: None,
            tags: vec![],
            parent_id: None,
        },
    )
    .unwrap();

    // タスクのステータスを変更
    diesel::update(tasks::table.find(&active_task.id))
        .set(tasks::status.eq("active"))
        .execute(&mut conn)
        .unwrap();

    diesel::update(tasks::table.find(&completed_task.id))
        .set(tasks::status.eq("completed"))
        .execute(&mut conn)
        .unwrap();

    diesel::update(tasks::table.find(&archived_task.id))
        .set(tasks::status.eq("archived"))
        .execute(&mut conn)
        .unwrap();

    // デフォルト（status = None）: Draft + Active のみ（REQ-0019）
    let default_tasks = TaskService::list_tasks(&mut conn, None).unwrap();
    assert_eq!(default_tasks.len(), 2);
    let default_ids: Vec<String> = default_tasks.iter().map(|t| t.id.clone()).collect();
    assert!(default_ids.contains(&draft_task.id));
    assert!(default_ids.contains(&active_task.id));

    // status = ["completed"]: Completedのみ
    let completed_tasks =
        TaskService::list_tasks(&mut conn, Some(vec!["completed".to_string()])).unwrap();
    assert_eq!(completed_tasks.len(), 1);
    assert_eq!(completed_tasks[0].id, completed_task.id);

    // status = ["archived"]: Archivedのみ
    let archived_tasks =
        TaskService::list_tasks(&mut conn, Some(vec!["archived".to_string()])).unwrap();
    assert_eq!(archived_tasks.len(), 1);
    assert_eq!(archived_tasks[0].id, archived_task.id);

    // status = ["draft", "active"]: Draft + Active
    let draft_active_tasks = TaskService::list_tasks(
        &mut conn,
        Some(vec!["draft".to_string(), "active".to_string()]),
    )
    .unwrap();
    assert_eq!(draft_active_tasks.len(), 2);

    // status = ["draft", "completed", "archived"]: Draft + Completed + Archived
    let mixed_tasks = TaskService::list_tasks(
        &mut conn,
        Some(vec![
            "draft".to_string(),
            "completed".to_string(),
            "archived".to_string(),
        ]),
    )
    .unwrap();
    assert_eq!(mixed_tasks.len(), 3);

    // status = []: 空配列 → 空の結果
    let empty_tasks = TaskService::list_tasks(&mut conn, Some(vec![])).unwrap();
    assert_eq!(empty_tasks.len(), 0);
}

// ===== ページネーション機能の統合テスト (REQ-0024) =====

#[test]
fn test_list_tasks_paginated_paging_behavior() {
    use tms_v2_lib::models::task::ListTasksPaginatedParams;

    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // 50個のタスクを作成（Draft状態）
    for i in 1..=50 {
        TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: format!("Test Task {}", i),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();
    }

    // 1ページ目（limit=20, offset=0）
    let page1 = TaskService::list_tasks_paginated(
        &mut conn,
        ListTasksPaginatedParams {
            status: None,
            limit: Some(20),
            offset: Some(0),
        },
    )
    .unwrap();

    assert_eq!(page1.tasks.len(), 20, "1ページ目は20件");
    assert_eq!(page1.total, 50, "総件数は50件");

    // 2ページ目（limit=20, offset=20）
    let page2 = TaskService::list_tasks_paginated(
        &mut conn,
        ListTasksPaginatedParams {
            status: None,
            limit: Some(20),
            offset: Some(20),
        },
    )
    .unwrap();

    assert_eq!(page2.tasks.len(), 20, "2ページ目は20件");
    assert_eq!(page2.total, 50, "総件数は50件");

    // 3ページ目（limit=20, offset=40）
    let page3 = TaskService::list_tasks_paginated(
        &mut conn,
        ListTasksPaginatedParams {
            status: None,
            limit: Some(20),
            offset: Some(40),
        },
    )
    .unwrap();

    assert_eq!(page3.tasks.len(), 10, "3ページ目は10件（残り）");
    assert_eq!(page3.total, 50, "総件数は50件");

    // ページ間で重複がないことを確認
    let all_ids: Vec<String> = page1
        .tasks
        .iter()
        .chain(page2.tasks.iter())
        .chain(page3.tasks.iter())
        .map(|t| t.id.clone())
        .collect();

    let mut unique_ids = all_ids.clone();
    unique_ids.sort();
    unique_ids.dedup();

    assert_eq!(
        unique_ids.len(),
        50,
        "全ページで重複なく50件取得できる"
    );
}

#[test]
fn test_list_tasks_paginated_total_count_with_filters() {
    use tms_v2_lib::models::task::{ListTasksPaginatedParams, UpdateTaskRequest};

    let pool = setup_test_pool();
    let mut conn = pool.get().expect("Failed to get connection");

    // Draft: 15個、Active: 10個、Completed: 8個、Archived: 5個
    for i in 1..=15 {
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

    for i in 1..=10 {
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
        use diesel::prelude::*;
        use tms_v2_lib::schema::tasks;
        diesel::update(tasks::table.find(&task.id))
            .set(tasks::status.eq("active"))
            .execute(&mut *conn)
            .unwrap();
    }

    for i in 1..=8 {
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
        use diesel::prelude::*;
        use tms_v2_lib::schema::tasks;
        diesel::update(tasks::table.find(&task.id))
            .set(tasks::status.eq("completed"))
            .execute(&mut *conn)
            .unwrap();
    }

    for i in 1..=5 {
        let task = TaskService::create_task(
            &mut conn,
            CreateTaskRequest {
                title: format!("Archived Task {}", i),
                description: None,
                tags: vec![],
                parent_id: None,
            },
        )
        .unwrap();
        TaskService::delete_task(&mut conn, &task.id).unwrap();
    }

    // デフォルト（Draft + Active）の総件数確認
    let result_default =
        TaskService::list_tasks_paginated(&mut conn, ListTasksPaginatedParams::default())
            .unwrap();
    assert_eq!(result_default.total, 25, "Draft + Active = 25件");
    assert_eq!(result_default.tasks.len(), 20, "デフォルトlimit=20で20件取得");

    // Completedのみ
    let result_completed = TaskService::list_tasks_paginated(
        &mut conn,
        ListTasksPaginatedParams {
            status: Some(vec!["completed".to_string()]),
            limit: None,
            offset: None,
        },
    )
    .unwrap();
    assert_eq!(result_completed.total, 8, "Completed = 8件");
    assert_eq!(result_completed.tasks.len(), 8);

    // Archivedのみ
    let result_archived = TaskService::list_tasks_paginated(
        &mut conn,
        ListTasksPaginatedParams {
            status: Some(vec!["archived".to_string()]),
            limit: None,
            offset: None,
        },
    )
    .unwrap();
    assert_eq!(result_archived.total, 5, "Archived = 5件");
    assert_eq!(result_archived.tasks.len(), 5);

    // Draft + Completed + Archived（複数ステータス指定）
    let result_multi = TaskService::list_tasks_paginated(
        &mut conn,
        ListTasksPaginatedParams {
            status: Some(vec![
                "draft".to_string(),
                "completed".to_string(),
                "archived".to_string(),
            ]),
            limit: Some(100),
            offset: None,
        },
    )
    .unwrap();
    assert_eq!(result_multi.total, 28, "Draft + Completed + Archived = 28件");
    assert_eq!(result_multi.tasks.len(), 28);
}
