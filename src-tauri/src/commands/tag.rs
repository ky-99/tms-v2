use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tauri::State;

use crate::models::tag::{CreateTagRequest, Tag, UpdateTagRequest};
use crate::service::TagService;

/// データベース接続プール型
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// 全タグ取得
#[tauri::command]
pub fn list_tags(pool: State<DbPool>) -> Result<Vec<Tag>, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TagService::list_tags(&mut conn).map_err(|e| e.to_string())
}

/// タグ作成
#[tauri::command]
pub fn create_tag(pool: State<DbPool>, request: CreateTagRequest) -> Result<Tag, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TagService::create_tag(&mut conn, request).map_err(|e| e.to_string())
}

/// タグ更新
#[tauri::command]
pub fn update_tag(
    pool: State<DbPool>,
    tag_id: String,
    request: UpdateTagRequest,
) -> Result<Tag, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TagService::update_tag(&mut conn, &tag_id, request).map_err(|e| e.to_string())
}

/// タグ削除
#[tauri::command]
pub fn delete_tag(pool: State<DbPool>, tag_id: String) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TagService::delete_tag(&mut conn, &tag_id).map_err(|e| e.to_string())
}
