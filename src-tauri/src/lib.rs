pub mod command;
pub mod models;
pub mod query;

use chrono::Utc;
use models::{Report, ReportWithTagParams};
use once_cell::sync::OnceCell;
use query::{report_query::fetch_report_with_tag_by_report_id, tag_query::fetch_tags_by_report};
use rusqlite::{params, Connection};
use std::sync::Mutex;

use crate::models::{ReportWithTag, Tag};

pub static DB_CONN: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn establish_connection() -> Connection {
    let conn = Connection::open("storage.db").unwrap();
    return conn;
}

pub fn get_time_of_now() -> String {
    return Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
}

///

pub fn find_all_reports(
    conn: &Connection,
    tag_name: Option<String>,
    page: i32,
    count: i32,
    latest: bool,
) -> Vec<ReportWithTag> {
    let mut query: Vec<String> = Vec::new();
    query.push("SELECT r.* FROM reports r".to_string());
    query.push("LEFT JOIN report_tags rt ON r.id = rt.report_id".to_string());
    query.push("LEFT JOIN tags t ON rt.tag_id = t.id".to_string());

    // タグ抽出
    if let Some(name) = tag_name {
        // joinしてタグ名で抽出
        query.push("WHERE t.name =".to_string());
        query.push("".to_string() + &"\"" + &name + &"\"");
    }

    // 並び替え
    let order = if latest { "ASC" } else { "DESC" };
    query.push("ORDER BY id".to_string());
    query.push(order.to_string());

    // ページネーション
    query.push("LIMIT".to_string());
    query.push(count.to_string());
    query.push("OFFSET".to_string());
    query.push(((page - 1) * count).to_string());

    // レポート配列の取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let reports = stmt
        .query_map([], |row| Report::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Report>>();

    // 各レポートにタグを紐づける
    let report_with_tags = reports
        .into_iter()
        .map(|report| {
            let tags = fetch_tags_by_report(conn, &report.id);
            return ReportWithTag::new(report, tags);
        })
        .collect::<Vec<ReportWithTag>>();

    return report_with_tags;
}

pub fn create_report(conn: &Connection, params: &ReportWithTagParams) -> ReportWithTag {
    let now = get_time_of_now();

    // レポート作成
    let _ = conn.execute(
        "
            INSERT INTO reports (title, body, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4)
        ",
        params![params.title, params.body, now.clone(), now],
    );

    // タグのバインド

    // 更新したレコードを取得
    let new_report = fetch_report_with_tag_by_report_id(conn, &conn.last_insert_rowid());
    return new_report;
}

// pub fn create_report(
//     conn: &mut SqliteConnection,
//     title: Option<String>,
//     body: String,
//     tag_names: Vec<String>,
// ) -> ReportWithTag {
//     // レポートを作成
//     let report = craete_report_returning(conn, title, body);

//     // タグ名を tag 配列に変換する
//     let tags = convert_tag_name_to_tag(conn, tag_names);

//     // タグを紐づける
//     associate_report_tag(conn, &report, &tags);

//     return ReportWithTag { report, tags };
// }

// pub fn update_report(
//     conn: &mut SqliteConnection,
//     report_id: &i32,
//     title: Option<String>,
//     body: String,
//     tag_names: Vec<String>,
// ) -> ReportWithTag {
//     // レポートを更新
//     let report = update_report_returning(conn, report_id, title, body);

//     // タグ名を tag 配列に変換する
//     let tags = convert_tag_name_to_tag(conn, tag_names);

//     // タグを紐づける
//     associate_report_tag(conn, &report, &tags);

//     return ReportWithTag { report, tags };
// }

// pub fn delete_report(conn: &mut SqliteConnection, report_id: &i32) -> bool {
//     // レポートを論理削除する
//     let _ = delete_report_returning(conn, report_id);

//     return true;
// }

// /// ////////////////////////////////////////////////////////////

// pub fn find_all_tags(conn: &mut SqliteConnection, has_pinned: bool) -> Vec<Tag> {
//     use crate::schema::tags as tag_schema;
//     use crate::schema::tags::dsl::id;
//     use crate::schema::tags::dsl::is_pinned;
//     use crate::schema::tags::dsl::tags;

//     let mut query = tags.into_boxed();
//     if has_pinned {
//         query = query.filter(tag_schema::is_pinned::eq(is_pinned, 1));
//     }

//     let db_tags = query.order_by(id.asc()).load::<Tag>(conn).unwrap();
//     return db_tags;
// }

// pub fn update_tag(
//     conn: &mut SqliteConnection,
//     tag_id: &i32,
//     name: String,
//     color: Option<String>,
//     is_pinned: i32,
//     priority: i32,
// ) -> Tag {
//     // レポートを更新
//     let tag = update_tag_returning(conn, tag_id, name, color, is_pinned, priority);

//     return tag;
// }
