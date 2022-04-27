#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{command, run_migration, DB_CONN};
use std::sync::Mutex;
use tauri::generate_handler;

fn main() {
    // init database
    let mut conn = app::establish_connection();
    run_migration(&mut conn);

    let _ = DB_CONN.set(Mutex::new(conn));

    // run tauri apptaur
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            command::report_get_all,
            command::report_create,
            command::report_update,
            command::report_remove,
            command::tag_get_all,
            command::tag_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
