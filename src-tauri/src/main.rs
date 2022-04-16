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
    let results = reports
        .limit(5)
        .load::<Report>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{:?}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
