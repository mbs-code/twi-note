use crate::models::{ReportWithTag, Tag};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::DB_CONN;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migration() {
    let mut conn = DB_CONN.get().unwrap().lock().unwrap();

    conn.run_pending_migrations(MIGRATIONS).unwrap();
    println!("migration");
}

/// ////////////////////////////////////////////////////////////

#[tauri::command]
pub fn report_get_all(page: i32, count: i32, latest: bool) -> Vec<ReportWithTag> {
    let mut conn = DB_CONN.get().unwrap().lock().unwrap();

    let reports = crate::find_all_reports(&mut conn, page, count, latest);
    return reports;
}

#[tauri::command]
pub fn report_create(title: Option<String>, body: String, tag_names: Vec<String>) -> ReportWithTag {
    let mut conn = DB_CONN.get().unwrap().lock().unwrap();

    let reports = crate::create_report(&mut conn, title, body, tag_names);
    return reports;
}

#[tauri::command]
pub fn report_update(
    id: i32,
    title: Option<String>,
    body: String,
    tag_names: Vec<String>,
) -> ReportWithTag {
    let mut conn = DB_CONN.get().unwrap().lock().unwrap();

    let reports = crate::update_report(&mut conn, id, title, body, tag_names);
    return reports;
}

#[tauri::command]
pub fn report_remove(id: i32) -> i32 {
    let mut conn = DB_CONN.get().unwrap().lock().unwrap();

    let _ = crate::delete_report(&mut conn, id);
    return id;
}
