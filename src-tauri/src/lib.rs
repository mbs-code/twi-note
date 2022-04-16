#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod query;
pub mod schema;

use chrono::Utc;
use diesel::sqlite::SqliteConnection;
use diesel::{dsl::sql, prelude::*};
use dotenv::dotenv;
use query::tag_query::convert_tag_name_to_tag;
use std::env;

use crate::models::{Report, ReportTagRecord};
use crate::query::report_tag_query::fetch_report_tag_by_report_id;
use crate::query::tag_query::convert_report_tag_to_tag;
use query::report_query::{craete_report_returning, update_report_returning};
use query::report_tag_query::associate_report_tag;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn find_all_reports(conn: &SqliteConnection) -> Vec<ReportTagRecord> {
    use crate::schema::reports::dsl::reports;

    let db_reports = reports.load::<Report>(conn).unwrap();

    let mut records: Vec<ReportTagRecord> = Vec::new();
    for db_report in db_reports {
        // report tag を取得
        let db_report_tags = fetch_report_tag_by_report_id(conn, db_report.id);

        // tag 二年関する
        let db_tags = convert_report_tag_to_tag(conn, db_report_tags);

        records.push(ReportTagRecord {
            report: db_report,
            tags: db_tags,
        });
    }

    return records;
}

pub fn create_report(
    conn: &SqliteConnection,
    title: Option<String>,
    body: String,
    tag_names: Vec<String>,
) -> ReportTagRecord {
    // レポートを作成
    let report = craete_report_returning(conn, title, body);

    // タグ名を tag 配列に変換する
    let tags = convert_tag_name_to_tag(conn, tag_names);

    // タグを紐づける
    associate_report_tag(conn, &report, &tags);

    return ReportTagRecord { report, tags };
}

pub fn update_report(
    conn: &SqliteConnection,
    report_id: i32,
    title: Option<String>,
    body: String,
    tag_names: Vec<String>,
) -> ReportTagRecord {
    // レポートを作成
    let report = update_report_returning(conn, report_id, title, body);

    // タグ名を tag 配列に変換する
    let tags = convert_tag_name_to_tag(conn, tag_names);

    // タグを紐づける
    associate_report_tag(conn, &report, &tags);

    return ReportTagRecord { report, tags };
}
