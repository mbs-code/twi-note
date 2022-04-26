pub mod command;
pub mod models;
pub mod query;

use once_cell::sync::OnceCell;
use rusqlite::Connection;
use std::sync::Mutex;

pub static DB_CONN: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn establish_connection() -> Connection {
    let conn = Connection::open("storage.db").unwrap();
    return conn;
}

// pub fn find_all_reports(
//     conn: &mut Connection,
//     tag_name: Option<String>,
//     page: i32,
//     count: i32,
// ) -> Vec<ReportWithTag> {
//     let mut query = "SELECT r.* FROM reports r".to_string();

//     // タグ抽出
//     if let Some(name) = tag_name {
//         // タグをIDに変換
//         let tag = fetch_tag_by_tag_name(conn, &name);

//         // 結合して検索
//         query.push_str(" LEFT JOIN report_tags rt ON r.id = rt.report_id WHERE rt.tag_id = ");
//         query.push_str(&tag.id.to_string());
//     }

//     // 検索条件
//     query.push_str(" LIMIT ");
//     query.push_str(&count.to_string());
//     query.push_str(" OFFSET ");
//     query.push_str(&((page - 1) * count).to_string());
//     query.push_str(" ORDER BY id DESC;");

//     let db_reports: Vec<Report> = sql_query(query).load(conn).unwrap();

//     let mut records: Vec<ReportWithTag> = Vec::new();
//     for db_report in db_reports {
//         // report tag を取得
//         let db_report_tags = fetch_report_tag_by_report_id(conn, db_report.id);

//         // tag に変換する
//         let db_tags = convert_report_tag_to_tag(conn, db_report_tags);

//         records.push(ReportWithTag {
//             report: db_report,
//             tags: db_tags,
//         });
//     }

//     return records;
// }

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
