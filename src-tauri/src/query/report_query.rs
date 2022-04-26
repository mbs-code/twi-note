use rusqlite::Connection;

use crate::models::{Report, ReportWithTag};

use super::tag_query::fetch_tags_by_report_id;

/** レポートIDを基準に、レポートを取得する */
pub fn fetch_report_with_tag_by_report_id(conn: &Connection, report_id: &i64) -> ReportWithTag {
    let mut query: Vec<String> = Vec::new();
    query.push("SELECT r.* FROM reports r".to_string());
    query.push("WHERE r.id =".to_string());
    query.push(report_id.to_string());
    query.push("ORDER BY id ASC".to_string());
    query.push("LIMIT 1".to_string());

    // レポート単体取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let report = stmt.query_row([], |row| Report::by_row(row)).unwrap();

    // レポートに紐づくタグを取得
    let tags = fetch_tags_by_report_id(conn, &report_id);
    let report_with_tag = ReportWithTag::new(report, tags);

    return report_with_tag;
}

// pub fn craete_report_returning(conn: &mut Connection, title: &Option<String>, body: &String) {
//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

//     let res = conn
//         .execute(
//             "INSERT INTO report (title, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
//             params![title, body, &now, &now],
//         )
//         .unwrap();

//     // diesel::insert_into(schema::reports::table)
//     //     .values(&new_report)
//     //     .execute(conn)
//     //     .expect("Error saving new post");

//     // let report = reports
//     //     .find(sql("last_insert_rowid()"))
//     //     .first::<Report>(conn)
//     //     .unwrap();
//     // return report;
// }

// pub fn update_report_returning(
//     conn: &mut SqliteConnection,
//     report_id: &i32,
//     title_val: Option<String>,
//     body_val: String,
// ) -> Report {
//     use crate::schema::reports::dsl::body;
//     use crate::schema::reports::dsl::reports;
//     use crate::schema::reports::dsl::title;
//     use crate::schema::reports::dsl::updated_at;

//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

//     diesel::update(reports.find(report_id))
//         .set((title.eq(title_val), body.eq(body_val), updated_at.eq(now)))
//         .execute(conn)
//         .unwrap();

//     let report = reports.find(report_id).first::<Report>(conn).unwrap();
//     return report;
// }

// pub fn delete_report_returning(conn: &mut SqliteConnection, report_id: &i32) -> Report {
//     use crate::schema::reports::dsl::deleted_at;
//     use crate::schema::reports::dsl::reports;
//     use crate::schema::reports::dsl::updated_at;

//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

//     diesel::update(reports.find(report_id))
//         .set((updated_at.eq(now.clone()), deleted_at.eq(now)))
//         .execute(conn)
//         .unwrap();

//     let report = reports.find(report_id).first::<Report>(conn).unwrap();
//     return report;
// }
