#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite, SqlitePool};

mod command;
use command::report_create;
use command::run_migration;

static DB_CONN: OnceCell<Pool<Sqlite>> = OnceCell::new();

#[async_std::main]
async fn main() {
    // DB connection
    let pool = SqlitePool::connect("sqlite:storage.db?mode=rwc")
        .await
        .unwrap();
    let _ = DB_CONN.set(pool);
    let _ = run_migration();

    // insert
    let report = report_create("タイトル".to_string(), "新規ぼでー".to_string()).await;
    println!("{:?}", report);

    // tauri::Builder::default()
    //   .run(tauri::generate_context!())
    //   .expect("error while running tauri application");
}
