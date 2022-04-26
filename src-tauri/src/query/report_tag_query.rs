use rusqlite::{params, Connection};

use crate::{get_time_of_now, models::ReportTagJoinTag};

use super::tag_query::fetch_tag_by_tag_name;

/** レポートIDを基準に、レポートタグ配列を取得 */
pub fn fetch_report_tags_by_report_id(conn: &Connection, report_id: &i64) -> Vec<ReportTagJoinTag> {
    let mut query: Vec<String> = Vec::new();
    query.push("SELECT rt.*, t.name as tag_name FROM report_tags rt".to_string());
    query.push("LEFT JOIN tags t ON rt.tag_id = t.id".to_string());
    query.push("WHERE rt.report_id =".to_string());
    query.push(report_id.to_string());
    query.push("ORDER BY id ASC".to_string());

    // レポートタグ配列の取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let report_tag_join_tag = stmt
        .query_map([], |row| ReportTagJoinTag::by_row(row))
        .unwrap()
        .map(|t| t.unwrap())
        .collect::<Vec<ReportTagJoinTag>>();

    return report_tag_join_tag;
}

pub fn associate_report_tag(conn: &Connection, report_id: &i64, tag_names: &Vec<String>) {
    // レポートに紐づいているタグを取得する
    let db_report_tags = fetch_report_tags_by_report_id(conn, report_id);

    // タグ名を回して、紐づいていないものを追加
    for tag_name in tag_names {
        let has = db_report_tags
            .iter()
            .filter(|&db_report_tag| tag_name.eq(&db_report_tag.tag_name))
            .count();

        // 一つも無いなら追加
        if has == 0 {
            attach_report_tag(conn, &report_id, &tag_name);
        }
    }

    // 逆に紐づきを回して、タグに無いものを削除
    for db_report_tag in db_report_tags {
        let has = tag_names
            .iter()
            .filter(|&tag_name| tag_name.eq(&db_report_tag.tag_name))
            .count();

        // 一つも無いなら削除
        if has == 0 {
            detach_report_tag(conn, &db_report_tag.id);
        }
    }
}

///

fn attach_report_tag(conn: &Connection, report_id: &i64, tag_name: &String) {
    // タグを取得
    let tag = fetch_tag_by_tag_name(conn, tag_name);

    // レポートタグ作成
    let now = get_time_of_now();
    let _ = conn.execute(
        "
            INSERT INTO report_tags (report_id, tag_id, created_at)
            VALUES (?1, ?2, ?3)
        ",
        params![report_id, &tag.id, now],
    );
}

fn detach_report_tag(conn: &Connection, report_tag_id: &i64) {
    // レポートタグ削除
    let _ = conn.execute(
        "
            DELETE FROM report_tags
            WHERE id = ?1
        ",
        params![report_tag_id],
    );
}

// fn detach_report_tag(conn: &mut SqliteConnection, report_tag: &ReportTag) {
//     use crate::schema::report_tags::dsl::report_tags;

//     diesel::delete(report_tags.find(report_tag.id))
//         .execute(conn)
//         .unwrap();
// }
