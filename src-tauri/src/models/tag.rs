use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::tags;

/// Tag エンティティ（DB SELECT結果 → API レスポンス用）
#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(table_name = tags)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub usage_count: i32,
    pub created_at: String, // ISO 8601形式の文字列
    // updated_at はレスポンスに含めない（OpenAPI仕様に未定義）
    #[serde(skip)]
    pub updated_at: String,
}

/// 新規タグ作成用（DB INSERT用）
#[derive(Debug, Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub usage_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl NewTag {
    /// CreateTagRequestから作成
    pub fn from_request(req: CreateTagRequest) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            color: req.color,
            usage_count: 0, // 初期値は0
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

/// タグ作成リクエスト（API受信用）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagRequest {
    pub name: String,
    #[serde(default)]
    pub color: Option<String>,
}

/// タグ更新リクエスト（API受信 + DB UPDATE用）
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = tags)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip)]
    pub updated_at: Option<String>,
}

impl UpdateTagRequest {
    /// updated_atタイムスタンプを設定
    pub fn with_timestamp(mut self) -> Self {
        self.updated_at = Some(Utc::now().to_rfc3339());
        self
    }
}
