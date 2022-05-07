pub mod command;
pub mod models;
pub mod query;

use chrono::Utc;
use once_cell::sync::OnceCell;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use serde::Serialize;
use std::sync::{Mutex, MutexGuard};
use tauri::Window;

pub static DB_CONN: OnceCell<Mutex<Connection>> = OnceCell::new();
pub static DB_PATH: &str = "./storage.db";

pub fn get_connection() -> MutexGuard<'static, Connection> {
    let conn = DB_CONN.get().unwrap().lock().unwrap();
    return conn;
}

pub fn establish_connection() -> Connection {
    let conn = Connection::open(DB_PATH).unwrap();
    return conn;
}

pub fn run_migration(conn: &mut Connection) {
    let migrations: Migrations = Migrations::new(vec![
        M::up(include_str!("../migrations/2022-04-16-021232_init/up.sql")).down(include_str!(
            "../migrations/2022-04-16-021232_init/down.sql"
        )),
        M::up(include_str!(
            "../migrations/2022-05-07-134912_create_phrase/up.sql"
        ))
        .down(include_str!(
            "../migrations/2022-05-07-134912_create_phrase/down.sql"
        )),
    ]);

    migrations.to_latest(conn).unwrap();
}

pub fn get_time_of_now() -> String {
    return Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
}

///

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}

// TODO: deprecated
pub fn fire_tag_changed(window: &Window) {
    window
        .emit("tag-changed", {
            Payload {
                message: "Change tag.".into(),
            }
        })
        .unwrap();
}

pub fn fire_phrase_changed(window: &Window) {
    window
        .emit("phrase-changed", {
            Payload {
                message: "Change phrase.".into(),
            }
        })
        .unwrap();
}
