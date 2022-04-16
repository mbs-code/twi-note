#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use once_cell::sync::OnceCell;
// use sqlx::{Pool, Sqlite, SqlitePool};
// use tauri::generate_handler;

// mod command;
// static DB_CONN: OnceCell<Pool<Sqlite>> = OnceCell::new();

use app;
use app::models::Report;
use app::schema::reports::dsl::reports;

use diesel::query_dsl::*;

fn main() {
    let conn = app::establish_connection();

    let create = app::create_report(
        &conn,
        Some("たいとる2".to_string()),
        "からだ".to_string(),
        vec![
            "タグ".to_string(),
            "新しい".to_string(),
            "などなど".to_string(),
        ],
    );
    println!("{:?}", create);

    let update = app::update_report(
        &conn,
        create.report.id,
        Some("変えたよ".to_string()),
        "こちらも".to_string(),
        vec!["変えたぞ".to_string(), "タグ".to_string()],
    );
    println!("{:?}", update);

    // let results = reports.load::<Report>(&conn).expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for report in results {
    //     println!("{:?}", report);
    // }
}
