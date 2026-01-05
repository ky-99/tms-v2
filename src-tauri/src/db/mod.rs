use rusqlite::{Connection, Result};
use rusqlite_migration::{Migrations, M};
use std::sync::{Arc, Mutex};
use tauri::{Manager, path::BaseDirectory, AppHandle};

use lazy_static::lazy_static;

// マイグレーション定義を lazy_static! で遅延初期化
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> = Migrations::new(vec![
        M::up("
CREATE TABLE tasks (
    id          TEXT PRIMARY KEY NOT NULL,
    title       TEXT NOT NULL,
    description TEXT,
    status      TEXT NOT NULL,
    parent_id   TEXT,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES tasks(id) ON DELETE SET NULL
);

CREATE TABLE tags (
    id          TEXT PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL UNIQUE,
    color       TEXT,
    usage_count INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

CREATE TABLE task_tags (
    task_id     TEXT NOT NULL,
    tag_id      TEXT NOT NULL,
    PRIMARY KEY (task_id, tag_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id)  REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE task_queue (
    task_id     TEXT PRIMARY KEY NOT NULL,
    position    INTEGER NOT NULL,
    added_at    TEXT NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE INDEX idx_tasks_status ON tasks (status);
CREATE INDEX idx_tasks_parent_id ON tasks (parent_id);
CREATE INDEX idx_tags_name ON tags (name);
CREATE INDEX idx_task_tags_task_id ON task_tags (task_id);
CREATE INDEX idx_task_tags_tag_id ON task_tags (tag_id);
CREATE INDEX idx_task_queue_position ON task_queue (position);
"),
    ]);
}

pub type DbConnection = Arc<Mutex<Connection>>;

// initialize_database 関数は tauri::AppHandle を受け取るように変更
pub fn initialize_database(app_handle: AppHandle) -> Result<DbConnection> {
    let db_path = get_db_path(&app_handle)?;
    let mut conn = Connection::open(db_path)?;

    MIGRATIONS.to_latest(&mut conn).map_err(|_e| {
        rusqlite::Error::InvalidQuery
    })?;

    Ok(Arc::new(Mutex::new(conn)))
}

// get_db_path 関数も tauri::AppHandle を受け取るように変更
pub fn get_db_path(app_handle: &AppHandle) -> Result<std::path::PathBuf> {
    let path_buf = app_handle.path().resolve(
        "tms-v2.db",
        BaseDirectory::AppData
    )
    .map_err(|_e| rusqlite::Error::InvalidQuery)?;

    // データベースファイルの親ディレクトリを作成（存在しない場合）
    if let Some(parent_dir) = path_buf.parent() {
        std::fs::create_dir_all(parent_dir)
            .map_err(|_e| rusqlite::Error::InvalidQuery)?;
    }

    Ok(path_buf)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_database_and_create_tables() {
        let mut conn = Connection::open_in_memory().unwrap();
        
        MIGRATIONS.to_latest(&mut conn).unwrap();

        // tasks テーブルが存在することを確認
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='tasks'").unwrap();
        assert!(stmt.exists([]).unwrap(), "tasks table should exist after migration");

        // tags テーブルが存在することを確認
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='tags'").unwrap();
        assert!(stmt.exists([]).unwrap(), "tags table should exist after migration");

        // task_tags テーブルが存在することを確認
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='task_tags'").unwrap();
        assert!(stmt.exists([]).unwrap(), "task_tags table should exist after migration");

        // tasks テーブルのカラムチェック (簡易チェック)
        let mut stmt = conn.prepare("PRAGMA table_info(tasks)").unwrap();
        let cols: Vec<String> = stmt.query_map([], |row| row.get(1)).unwrap().map(|c| c.unwrap()).collect();
        assert!(cols.contains(&"id".to_string()));
        assert!(cols.contains(&"title".to_string()));
        assert!(cols.contains(&"description".to_string()));
        assert!(cols.contains(&"status".to_string()));
        assert!(cols.contains(&"parent_id".to_string()));
        assert!(cols.contains(&"created_at".to_string()));
        assert!(cols.contains(&"updated_at".to_string()));

        // tags テーブルのカラムチェック (簡易チェック)
        let mut stmt = conn.prepare("PRAGMA table_info(tags)").unwrap();
        let cols: Vec<String> = stmt.query_map([], |row| row.get(1)).unwrap().map(|c| c.unwrap()).collect();
        assert!(cols.contains(&"id".to_string()));
        assert!(cols.contains(&"name".to_string()));
        assert!(cols.contains(&"color".to_string()));
        assert!(cols.contains(&"usage_count".to_string()));
        assert!(cols.contains(&"created_at".to_string()));
        assert!(cols.contains(&"updated_at".to_string()));
        
        // task_tags テーブルのカラムチェック (簡易チェック)
        let mut stmt = conn.prepare("PRAGMA table_info(task_tags)").unwrap();
        let cols: Vec<String> = stmt.query_map([], |row| row.get(1)).unwrap().map(|c| c.unwrap()).collect();
        assert!(cols.contains(&"task_id".to_string()));
        assert!(cols.contains(&"tag_id".to_string()));


        // インデックスが存在することを確認
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_tasks_status'").unwrap();
        assert!(stmt.exists([]).unwrap(), "idx_tasks_status should exist");

        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_tasks_parent_id'").unwrap();
        assert!(stmt.exists([]).unwrap(), "idx_tasks_parent_id should exist");

        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_tags_name'").unwrap();
        assert!(stmt.exists([]).unwrap(), "idx_tags_name should exist");

        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_task_tags_task_id'").unwrap();
        assert!(stmt.exists([]).unwrap(), "idx_task_tags_task_id should exist");

        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_task_tags_tag_id'").unwrap();
        assert!(stmt.exists([]).unwrap(), "idx_task_tags_tag_id should exist");
    }
}
