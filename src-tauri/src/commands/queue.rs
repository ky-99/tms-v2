use tauri::State;

use crate::models::queue::{
    AddToQueueRequest, CompleteAllQueueResponse, QueueEntry, QueueEntryWithTask,
    RemoveFromQueueRequest, ReorderQueueRequest, UpdateQueueRequest,
};
use crate::service::QueueService;
use crate::DbPool;

/// キュー全体を取得
#[tauri::command]
pub fn get_task_queue(pool: State<DbPool>) -> Result<Vec<QueueEntryWithTask>, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    QueueService::get_queue(&mut conn).map_err(|e| e.to_string())
}

/// タスクをキューに追加
#[tauri::command]
pub fn add_task_to_queue(
    pool: State<DbPool>,
    req: AddToQueueRequest,
) -> Result<QueueEntry, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    QueueService::add_to_queue(&mut conn, req.task_id).map_err(|e| e.to_string())
}

/// タスクをキューから削除
#[tauri::command]
pub fn remove_task_from_queue(
    pool: State<DbPool>,
    req: RemoveFromQueueRequest,
) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    QueueService::remove_from_queue(&mut conn, req.task_id, req.target_status)
        .map_err(|e| e.to_string())
}

/// キュー全体をクリア
#[tauri::command]
pub fn clear_task_queue(pool: State<DbPool>) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    QueueService::clear_queue(&mut conn).map_err(|e| e.to_string())
}

/// キュー内の全タスクを完了状態にする
#[tauri::command]
pub fn complete_all_queue(pool: State<DbPool>) -> Result<CompleteAllQueueResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    let completed_count = QueueService::complete_all_queue(&mut conn).map_err(|e| e.to_string())?;
    Ok(CompleteAllQueueResponse { completed_count })
}

/// タスクのキュー内位置を更新
#[tauri::command]
pub fn update_queue_position(
    pool: State<DbPool>,
    req: UpdateQueueRequest,
) -> Result<QueueEntry, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    QueueService::update_queue_position(&mut conn, req.task_id, req.new_position)
        .map_err(|e| e.to_string())
}

/// キュー全体を一括で並び替え
#[tauri::command]
pub fn reorder_task_queue(
    pool: State<DbPool>,
    req: ReorderQueueRequest,
) -> Result<Vec<QueueEntry>, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    QueueService::reorder_queue(&mut conn, req.task_ids).map_err(|e| e.to_string())
}
