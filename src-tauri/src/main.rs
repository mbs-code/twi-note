#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use sqlx::{migrate::Migrator, SqlitePool};
use std::path::Path;

#[async_std::main]
async fn main() {
    // DB connection
    let pool = SqlitePool::connect("sqlite:storage.db?mode=rwc")
        .await
        .unwrap();

    let m = Migrator::new(Path::new("./migrations")).await.unwrap();
    println!("{:?}", m);

    let _ = m.run(&pool).await.unwrap();
    println!("{}", "OK");

    // tauri::Builder::default()
    //   .run(tauri::generate_context!())
    //   .expect("error while running tauri application");
}
