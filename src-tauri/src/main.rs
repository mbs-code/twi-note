#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{
    create_report, find_all_reports, models::ReportWithTagParams,
    query::tag_query::fetch_tag_by_tag_name, update_report, DB_CONN,
};
use std::sync::Mutex;
// use tauri::generate_handler;

fn main() {
    // init database
    let conn = app::establish_connection();
    // let _ = DB_CONN.set(Mutex::new(conn));

    // fetch_tag_by_tag_name(&conn, &"タグ".to_string());

    // let rcparams = ReportWithTagParams {
    //     title: Some("新規タイトル".to_string()),
    //     body: "新規タグ".to_string(),
    //     tag_names: vec!["テスト".to_string(), "タグ".to_string()],
    // };
    // let add = create_report(&conn, &rcparams);
    // println!("{:?}", add);

    let res = find_all_reports(&conn, None, 1, 1, true);
    println!("{:?}", res);

    let rcparams = ReportWithTagParams {
        title: Some("かえて".to_string()),
        body: "みた".to_string(),
        tag_names: vec!["テスト".to_string(), "タグ".to_string()],
    };
    let add = update_report(&conn, &18, &rcparams);
    println!("{:?}", add);

    let res = find_all_reports(&conn, None, 1, 1, true);
    println!("{:?}", res);

    // command::run_migration();

    // // run tauri apptaur
    // tauri::Builder::default()
    //     .invoke_handler(generate_handler![
    //         command::report_get_all,
    //         command::report_create,
    //         command::report_update,
    //         command::report_remove,
    //         command::tag_get_all,
    //         command::tag_update,
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
