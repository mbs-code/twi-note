use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models::ReportWithTag;

use crate::DB_CONN;

// pub async fn run_migration() {
//     let npool = DB_CONN.get().unwrap();
//     let _ = sqlx::migrate!("./migrations").run(npool).await.unwrap();
//     println!("migration");
// }

#[tauri::command]
pub fn report_get_all(page: i32, count: i32, latest: bool) -> Vec<ReportWithTag> {
    let conn = DB_CONN.get().unwrap().lock().unwrap();

    let reports = crate::find_all_reports(&conn, page, count, latest);
    return reports;
}

#[tauri::command]
pub fn report_create(title: Option<String>, body: String, tag_names: Vec<String>) -> ReportWithTag {
    let conn = DB_CONN.get().unwrap().lock().unwrap();

    let reports = crate::create_report(&conn, title, body, tag_names);
    return reports;
}

#[tauri::command]
pub fn report_update(
    id: i32,
    title: Option<String>,
    body: String,
    tag_names: Vec<String>,
) -> ReportWithTag {
    let conn = DB_CONN.get().unwrap().lock().unwrap();

    let reports = crate::update_report(&conn, id, title, body, tag_names);
    return reports;
}

#[tauri::command]
pub fn report_remove(id: i32) -> i32 {
    let conn = DB_CONN.get().unwrap().lock().unwrap();

    let _ = crate::delete_report(&conn, id);
    return id;
}
