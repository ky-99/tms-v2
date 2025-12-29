use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::tasks;

/// タスクのステータス
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Draft,
    Active,
    Completed,
    Archived,
}

impl TaskStatus {
    /// ステータスを文字列に変換
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Draft => "draft",
            TaskStatus::Active => "active",
            TaskStatus::Completed => "completed",
            TaskStatus::Archived => "archived",
        }
    }

    /// 文字列からステータスをパース
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "draft" => Some(TaskStatus::Draft),
            "active" => Some(TaskStatus::Active),
            "completed" => Some(TaskStatus::Completed),
            "archived" => Some(TaskStatus::Archived),
            _ => None,
        }
    }
}

/// Task エンティティ（DB SELECT結果 → API レスポンス用）
#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(table_name = tasks)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip)]
    pub status: String, // DBではTEXT型（内部用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub created_at: String, // ISO 8601形式の文字列
    pub updated_at: String, // ISO 8601形式の文字列
}

impl Task {
    /// ステータス文字列をTaskStatus enumに変換
    pub fn status_enum(&self) -> TaskStatus {
        TaskStatus::from_str(&self.status).unwrap_or(TaskStatus::Draft)
    }

    /// tagsフィールドを追加したレスポンス用の構造体に変換
    pub fn with_tags(self, tags: Vec<String>) -> TaskResponse {
        let status = self.status_enum(); // 先にステータスを取得
        TaskResponse {
            id: self.id,
            title: self.title,
            description: self.description,
            status,
            tags,
            parent_id: self.parent_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            children_ids: Vec::new(), // 初期化（後でサービス層で設定）
        }
    }
}

/// Task レスポンス（tagsフィールド付き）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: TaskStatus,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub children_ids: Vec<String>, // 子タスクのIDリスト
}

/// 階層構造を持つタスクレスポンス（get_hierarchy API用）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskHierarchyResponse {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: TaskStatus,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub children: Vec<TaskHierarchyResponse>, // 子タスクの配列（再帰的）
}

/// 新規タスク作成用（DB INSERT用）
#[derive(Debug, Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub parent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl NewTask {
    /// CreateTaskRequestから作成
    pub fn from_request(req: CreateTaskRequest) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: req.title,
            description: req.description,
            status: TaskStatus::Draft.as_str().to_string(),
            parent_id: req.parent_id,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

/// タスク作成リクエスト（API受信用）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
}

/// タスク更新リクエスト（API受信用）
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskRequestInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

/// タスク更新リクエスト（DB UPDATE用）
#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    #[diesel(column_name = status)]
    pub status: Option<String>,
    pub parent_id: Option<String>,
    pub updated_at: Option<String>,
}

impl UpdateTaskRequest {
    /// ステータス更新用のヘルパー
    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = Some(status.as_str().to_string());
        self
    }

    /// updated_atタイムスタンプを設定
    pub fn with_timestamp(mut self) -> Self {
        self.updated_at = Some(Utc::now().to_rfc3339());
        self
    }
}

/// タスク検索パラメータ（API受信用）
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTasksParams {
    /// キーワード検索（タイトル・説明文）
    #[serde(default)]
    pub q: Option<String>,
    /// ステータスフィルタ
    #[serde(default)]
    pub status: Option<String>,
    /// タグフィルタ（OR条件）
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

/// タスク一覧取得（ページネーション対応）パラメータ（API受信用）
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTasksPaginatedParams {
    /// ステータスフィルタ（複数指定可能）
    #[serde(default)]
    pub status: Option<Vec<String>>,
    /// 取得件数（デフォルト20）
    #[serde(default)]
    pub limit: Option<i64>,
    /// オフセット（デフォルト0）
    #[serde(default)]
    pub offset: Option<i64>,
}

/// ページネーション付きタスク一覧レスポンス
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedTaskResponse {
    /// タスクリスト
    pub tasks: Vec<TaskResponse>,
    /// 総件数（フィルタ適用後）
    pub total: i64,
}
