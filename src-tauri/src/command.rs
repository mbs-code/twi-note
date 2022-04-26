// const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

// pub fn run_migration() {
//     let mut conn = DB_CONN.get().unwrap().lock().unwrap();

//     conn.run_pending_migrations(MIGRATIONS).unwrap();
//     println!("migration");
// }

// #[tauri::command]
// pub fn report_get_all(tag_name: Option<String>, page: i32, count: i32) -> Vec<ReportWithTag> {
//     let mut conn = DB_CONN.get().unwrap().lock().unwrap();

//     let reports = crate::find_all_reports(&mut conn, tag_name, page, count);
//     return reports;
// }

// #[tauri::command]
// pub fn report_create(title: Option<String>, body: String, tag_names: Vec<String>) -> ReportWithTag {
//     let mut conn = DB_CONN.get().unwrap().lock().unwrap();

//     let reports = crate::create_report(&mut conn, title, body, tag_names);
//     return reports;
// }

// #[tauri::command]
// pub fn report_update(
//     id: i32,
//     title: Option<String>,
//     body: String,
//     tag_names: Vec<String>,
// ) -> ReportWithTag {
//     let mut conn = DB_CONN.get().unwrap().lock().unwrap();

//     let report = crate::update_report(&mut conn, &id, title, body, tag_names);
//     return report;
// }

// #[tauri::command]
// pub fn report_remove(id: i32) -> i32 {
//     let mut conn = DB_CONN.get().unwrap().lock().unwrap();

//     let _ = crate::delete_report(&mut conn, &id);
//     return id;
// }

// /// ////////////////////////////////////////////////////////////

// #[tauri::command]
// pub fn tag_get_all(has_pinned: bool) -> Vec<Tag> {
//     let mut conn = DB_CONN.get().unwrap().lock().unwrap();

//     let db_tags = crate::find_all_tags(&mut conn, has_pinned);
//     return db_tags;
// }

// #[tauri::command]
// pub fn tag_update(
//     id: i32,
//     name: String,
//     color: Option<String>,
//     is_pinned: bool,
//     priority: i32,
// ) -> Tag {
//     let mut conn = DB_CONN.get().unwrap().lock().unwrap();

//     let is_pinned_num = if is_pinned { 1 } else { 0 };
//     let tag = crate::update_tag(&mut conn, &id, name, color, is_pinned_num, priority);
//     return tag;
// }
