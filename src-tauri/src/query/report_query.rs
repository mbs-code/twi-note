use rusqlite::Connection;

use super::tag_query::fetch_tags_by_report_id;
use crate::models::{Report, ReportWithTag};

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
