#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite, SqlitePool};
use tauri::generate_handler;

mod command;
static DB_CONN: OnceCell<Pool<Sqlite>> = OnceCell::new();

#[tokio::main]
async fn main() {
    // init database
    let pool = SqlitePool::connect("sqlite:storage.db?mode=rwc")
        .await
        .unwrap();
    let _ = DB_CONN.set(pool);
    let _ = command::run_migration().await;

    // run tauri apptaur
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            command::report_get_all,
            command::report_create,
            command::report_update,
            command::report_remove,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
