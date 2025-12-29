use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tauri::State;

use crate::error::ServiceError;
use crate::models::task::{
    CreateTaskRequest, ListTasksPaginatedParams, PaginatedTaskResponse, SearchTasksParams,
    TaskHierarchyResponse, TaskResponse, UpdateTaskRequest,
};
use crate::service::TaskService;

/// データベース接続プール型
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// ServiceErrorを分かりやすい日本語メッセージに変換
fn format_error(err: ServiceError) -> String {
    match err {
        ServiceError::TaskNotFound(id) => format!("タスクが見つかりません（ID: {}）", id),
        ServiceError::ParentTaskNotFound(id) => {
            format!("指定された親タスクが見つかりません（ID: {}）", id)
        }
        ServiceError::InvalidInput(msg) => format!("入力値が不正です: {}", msg),
        ServiceError::CircularDependency(id) => {
            format!("循環参照が検出されました。タスク {} を親に設定できません", id)
        }
        ServiceError::TaskHasChildren(id) => {
            format!(
                "子タスクが存在するため削除できません（タスクID: {}）",
                id
            )
        }
        ServiceError::TaskNotDraft(id) => {
            format!(
                "Draft状態のタスクのみ編集・削除できます（タスクID: {}）",
                id
            )
        }
        ServiceError::TaskNotArchived(id) => {
            format!(
                "Archived状態のタスクのみ物理削除できます（タスクID: {}）",
                id
            )
        }
        ServiceError::DatabaseError(e) => format!("データベースエラー: {}", e),
        _ => err.to_string(),
    }
}

/// タスクを作成
#[tauri::command]
pub fn create_task(
    pool: State<DbPool>,
    req: CreateTaskRequest,
) -> Result<TaskResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::create_task(&mut conn, req).map_err(format_error)
}

/// タスクを取得
#[tauri::command]
pub fn get_task(pool: State<DbPool>, task_id: String) -> Result<TaskResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::get_task(&mut conn, &task_id).map_err(format_error)
}

/// タスクを更新
#[tauri::command]
pub fn update_task(
    pool: State<DbPool>,
    task_id: String,
    req: UpdateTaskRequest,
) -> Result<TaskResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::update_task(&mut conn, &task_id, req).map_err(format_error)
}

/// タスクを削除（論理削除: Draft → Archived）
#[tauri::command]
pub fn delete_task(pool: State<DbPool>, task_id: String) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::delete_task(&mut conn, &task_id).map_err(format_error)
}

/// タスクを完全に削除（物理削除: データベースから削除）
///
/// # Notes
/// - Archivedステータスのタスクのみ物理削除可能
/// - 子タスクも自動的に削除される（CASCADE）
#[tauri::command]
pub fn delete_task_permanently(pool: State<DbPool>, task_id: String) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::delete_task_permanently(&mut conn, &task_id).map_err(format_error)
}

/// タスクを復元（Archived → Draft）
///
/// # Notes
/// - Archivedステータスのタスクのみ復元可能
/// - タスクのステータスをDraftに変更し、updated_atを更新する
#[tauri::command]
pub fn restore_task(pool: State<DbPool>, task_id: String) -> Result<TaskResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::restore_task(&mut conn, &task_id).map_err(format_error)
}

/// タスク一覧を取得（ステータスフィルタ対応）
///
/// # Parameters
/// * `status` - ステータスフィルタ（配列形式、オプション）
///   - None: Draft + Active（デフォルト）
///   - Some(["completed"]): Completedのみ
///   - Some(["archived"]): Archivedのみ
///   - Some(["draft", "active"]): Draft + Active
#[tauri::command]
pub fn list_tasks(
    pool: State<DbPool>,
    status: Option<Vec<String>>,
) -> Result<Vec<TaskResponse>, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::list_tasks(&mut conn, status).map_err(format_error)
}

/// タスク一覧を取得（ページネーション対応）
///
/// # Parameters
/// * `params` - ページネーションパラメータ
///   - status: ステータスフィルタ（オプション、デフォルト: Draft + Active）
///   - limit: 取得件数（オプション、デフォルト: 20）
///   - offset: オフセット（オプション、デフォルト: 0）
///
/// # Returns
/// * `PaginatedTaskResponse` - タスクリストと総件数
///
/// # Usage
/// - CompletedPage/ArchivedPageで使用
/// - REQ-0024: ページネーション機能実装
#[tauri::command]
pub fn list_tasks_paginated(
    pool: State<DbPool>,
    params: ListTasksPaginatedParams,
) -> Result<PaginatedTaskResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::list_tasks_paginated(&mut conn, params).map_err(format_error)
}

/// タスク階層を取得（Draft/Active な親 + Draft/Active/Completed な子）
#[tauri::command]
pub fn get_task_hierarchy(
    pool: State<DbPool>,
) -> Result<Vec<TaskHierarchyResponse>, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TaskService::get_hierarchy(&mut conn).map_err(format_error)
}

/// タスク検索（フィルタ・キーワード対応）
#[tauri::command]
pub fn search_tasks(
    pool: State<DbPool>,
    q: Option<String>,
    status: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Vec<TaskResponse>, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    let params = SearchTasksParams { q, status, tags };
    TaskService::search_tasks(&mut conn, params).map_err(format_error)
}
