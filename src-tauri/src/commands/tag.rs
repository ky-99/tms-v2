use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tauri::State;

use crate::error::ServiceError;
use crate::models::tag::{CreateTagRequest, Tag, UpdateTagRequest};
use crate::service::TagService;

/// データベース接続プール型
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// ServiceErrorを分かりやすい日本語メッセージに変換
fn format_error(err: ServiceError) -> String {
    match err {
        ServiceError::TagNotFound(id) => format!("タグが見つかりません（ID: {}）", id),
        ServiceError::TagInUse(id) => {
            format!("タグは使用中のため削除できません（タグID: {}）", id)
        }
        ServiceError::InvalidInput(msg) => format!("入力値が不正です: {}", msg),
        ServiceError::DatabaseError(e) => format!("データベースエラー: {}", e),
        _ => err.to_string(),
    }
}

/// 全タグ取得
#[tauri::command]
pub fn list_tags(pool: State<DbPool>) -> Result<Vec<Tag>, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TagService::list_tags(&mut conn).map_err(format_error)
}

/// タグ作成
#[tauri::command]
pub fn create_tag(pool: State<DbPool>, request: CreateTagRequest) -> Result<Tag, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TagService::create_tag(&mut conn, request).map_err(format_error)
}

/// タグ更新
#[tauri::command]
pub fn update_tag(
    pool: State<DbPool>,
    tag_id: String,
    request: UpdateTagRequest,
) -> Result<Tag, String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TagService::update_tag(&mut conn, &tag_id, request).map_err(format_error)
}

/// タグ削除
#[tauri::command]
pub fn delete_tag(pool: State<DbPool>, tag_id: String) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("データベース接続エラー: {}", e))?;
    TagService::delete_tag(&mut conn, &tag_id).map_err(format_error)
}
