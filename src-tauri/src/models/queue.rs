use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::task_queue;

/// タスクキューエントリ（データベースモデル）
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = task_queue)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct QueueEntry {
    pub task_id: String,
    pub position: i32,
    pub added_at: String,
}

/// 新規キューエントリ（挿入用）
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = task_queue)]
pub struct NewQueueEntry {
    pub task_id: String,
    pub position: i32,
    pub added_at: String,
}

impl NewQueueEntry {
    pub fn new(task_id: String, position: i32) -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            task_id,
            position,
            added_at: now.to_rfc3339(),
        }
    }
}

/// タスクキューエントリ（タスク情報含む）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueEntryWithTask {
    pub task_id: String,
    pub position: i32,
    pub added_at: String,
    pub task_title: String,
    pub task_status: String,
    pub task_description: Option<String>,
}

/// タスクキュー追加リクエスト
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToQueueRequest {
    pub task_id: String,
}

/// タスクキュー削除リクエスト
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromQueueRequest {
    pub task_id: String,
    pub target_status: String, // "draft" または "completed"
}

/// タスクキュー更新リクエスト（順序変更用）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQueueRequest {
    pub task_id: String,
    pub new_position: i32,
}

/// タスクキュー一括更新リクエスト
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderQueueRequest {
    pub task_ids: Vec<String>, // 新しい順序でのタスクIDリスト
}
