#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod command;
pub mod models;
pub mod query;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use query::tag_query::convert_tag_name_to_tag;
use std::env;
use std::sync::Mutex;

use crate::models::{Report, ReportWithTag};
use crate::query::report_tag_query::fetch_report_tag_by_report_id;
use crate::query::tag_query::convert_report_tag_to_tag;
use query::report_query::{
    craete_report_returning, delete_report_returning, update_report_returning,
};
use query::report_tag_query::associate_report_tag;

pub static DB_CONN: OnceCell<Mutex<SqliteConnection>> = OnceCell::new();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn find_all_reports(
    conn: &mut SqliteConnection,
    page: i32,
    count: i32,
    latest: bool,
) -> Vec<ReportWithTag> {
    use crate::schema::reports::dsl::id;
    use crate::schema::reports::dsl::reports;

    let db_reports = reports
        .limit(count.into())
        .offset(((page - 1) * count).into())
        .order_by(id.desc())
        .load::<Report>(conn)
        .unwrap();

    let mut records: Vec<ReportWithTag> = Vec::new();
    for db_report in db_reports {
        // report tag を取得
        let db_report_tags = fetch_report_tag_by_report_id(conn, db_report.id);

        // tag に変換する
        let db_tags = convert_report_tag_to_tag(conn, db_report_tags);

        records.push(ReportWithTag {
            report: db_report,
            tags: db_tags,
        });
    }

    return records;
}

pub fn create_report(
    conn: &mut SqliteConnection,
    title: Option<String>,
    body: String,
    tag_names: Vec<String>,
) -> ReportWithTag {
    // レポートを作成
    let report = craete_report_returning(conn, title, body);

    // タグ名を tag 配列に変換する
    let tags = convert_tag_name_to_tag(conn, tag_names);

    // タグを紐づける
    associate_report_tag(conn, &report, &tags);

    return ReportWithTag { report, tags };
}

pub fn update_report(
    conn: &mut SqliteConnection,
    report_id: i32,
    title: Option<String>,
    body: String,
    tag_names: Vec<String>,
) -> ReportWithTag {
    // レポートを更新
    let report = update_report_returning(conn, report_id, title, body);

    // タグ名を tag 配列に変換する
    let tags = convert_tag_name_to_tag(conn, tag_names);

    // タグを紐づける
    associate_report_tag(conn, &report, &tags);

    return ReportWithTag { report, tags };
}

pub fn delete_report(conn: &mut SqliteConnection, report_id: i32) -> bool {
    // レポートを論理削除する
    let _ = delete_report_returning(conn, report_id);

    return true;
}
