// Suppress warnings from macro expansions in external crates
#![allow(unexpected_cfgs)]

// モジュール宣言
pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod schema;
pub mod service;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

use diesel::r2d2::{ConnectionManager, Pool, CustomizeConnection};
use diesel::SqliteConnection;
use std::path::PathBuf;
use tauri::Manager;
use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

/// データベース接続プール型
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// FOREIGN KEY制約を有効化するカスタマイザ
///
/// SQLiteはデフォルトでFOREIGN KEY制約が無効になっているため、
/// 各接続で明示的に有効化する必要がある。
/// これにより、CASCADE削除などの参照整合性制約が正しく動作する。
#[derive(Debug, Clone, Copy)]
struct ForeignKeyEnabler;

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ForeignKeyEnabler {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        use diesel::RunQueryDsl;

        // FOREIGN KEY制約を有効化
        diesel::sql_query("PRAGMA foreign_keys = ON;")
            .execute(conn)
            .map_err(diesel::r2d2::Error::QueryError)?;

        Ok(())
    }
}

/// データベース接続プールを初期化
fn init_db_pool(db_path: PathBuf) -> Result<DbPool, String> {
    let database_url = db_path.to_str().ok_or("Invalid database path")?;
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .connection_customizer(Box::new(ForeignKeyEnabler))
        .build(manager)
        .map_err(|e| format!("Failed to create pool: {}", e))
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(unexpected_cfgs)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // データベースパスを取得
            let db_path = db::get_db_path(app.handle())?;

            // データベース初期化（既存のdb::initialize_database関数を使用）
            db::initialize_database(app.handle().clone())?;

            // 接続プールを作成
            let pool = init_db_pool(db_path).map_err(|e| e.to_string())?;

            // アプリケーションステートに接続プールを登録
            app.manage(pool);

            // Create main window with transparent title bar
            let mut window_builder = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::default()
            )
            .title("TasQue")
            .inner_size(880.0, 660.0)
            .min_inner_size(800.0, 600.0)
            .resizable(true)
            .transparent(true)
            .hidden_title(true);

            #[cfg(target_os = "macos")]
            {
                window_builder = window_builder.title_bar_style(TitleBarStyle::Overlay);
            }

            let window = window_builder.build()?;

            #[cfg(target_os = "macos")]
            {
                #[allow(deprecated)]
                use cocoa::appkit::{NSWindow, NSWindowButton, NSColor};
                #[allow(deprecated)]
                use cocoa::base::{id, nil, YES};

                #[allow(deprecated, unexpected_cfgs)]
                unsafe {
                    let ns_window = window.ns_window().unwrap() as id;

                    // Set transparent background
                    ns_window.setBackgroundColor_(NSColor::clearColor(nil));

                    // Hide window buttons (close, minimize, maximize)
                    let close_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
                    let miniaturize_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
                    let zoom_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowZoomButton);

                    if !close_button.is_null() {
                        let _: () = msg_send![close_button, setHidden: YES];
                    }
                    if !miniaturize_button.is_null() {
                        let _: () = msg_send![miniaturize_button, setHidden: YES];
                    }
                    if !zoom_button.is_null() {
                        let _: () = msg_send![zoom_button, setHidden: YES];
                    }

                    // Make title bar shorter
                    ns_window.setTitlebarAppearsTransparent_(YES);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Utility
            greet,
            // Task Management (12 commands)
            // - CRUD operations for tasks
            // - Search and filter functionality (including lightweight ID-only search)
            // - Hierarchical task retrieval
            // - Physical deletion and restore for archived tasks
            // - Pagination support for list operations
            // - Task duplication with recursive child handling
            commands::task::create_task,
            commands::task::duplicate_task,
            commands::task::get_task,
            commands::task::update_task,
            commands::task::delete_task,
            commands::task::delete_task_permanently,
            commands::task::restore_task,
            commands::task::list_tasks,
            commands::task::list_tasks_paginated,
            commands::task::get_task_hierarchy,
            commands::task::search_tasks,
            commands::task::search_task_ids,
            // Task Queue Management (7 commands)
            // - Daily task queue operations
            // - Position management and reordering
            // - Batch operations (complete all, clear all)
            commands::queue::get_task_queue,
            commands::queue::add_task_to_queue,
            commands::queue::remove_task_from_queue,
            commands::queue::clear_task_queue,
            commands::queue::complete_all_queue,
            commands::queue::update_queue_position,
            commands::queue::reorder_task_queue,
            // Tag Management (4 commands)
            // - Tag CRUD operations
            // - Usage count tracking
            commands::tag::list_tags,
            commands::tag::create_tag,
            commands::tag::update_tag,
            commands::tag::delete_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
