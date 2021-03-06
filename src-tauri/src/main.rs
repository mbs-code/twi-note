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
            command::report_command::report_get_all,
            command::report_command::report_create,
            command::report_command::report_update,
            command::report_command::report_remove,
            command::tag_command::tag_get_all,
            command::tag_command::tag_update,
            command::phrase_command::phrase_get_all,
            command::phrase_command::phrase_create,
            command::phrase_command::phrase_update,
            command::phrase_command::phrase_remove,
            command::config_command::load_config,
            command::config_command::save_config,
            command::storage_command::get_storage_info,
            command::storage_command::open_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
