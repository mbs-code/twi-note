use rusqlite::{params, Connection};

use crate::{get_time_of_now, models::Tag};

/** レポートIDを基準に、タグ配列を取得 */
pub fn fetch_tags_by_report_id(conn: &Connection, report_id: &i64) -> Vec<Tag> {
    let mut query: Vec<String> = Vec::new();
    query.push("SELECT t.* FROM tags t".to_string());
    query.push("LEFT JOIN report_tags rt ON t.id = rt.tag_id".to_string());
    query.push("WHERE rt.report_id =".to_string());
    query.push(report_id.to_string());
    query.push("ORDER BY priority DESC, id ASC".to_string());

    // タグ配列取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let tags = stmt
        .query_map([], |row| Tag::by_row(row))
        .unwrap()
        .map(|t| t.unwrap())
        .collect::<Vec<Tag>>();

    return tags;
}

/** タグIDを基準に、タグを取得する */
pub fn fetch_tag_by_tag_id(conn: &Connection, tag_id: &i64) -> Option<Tag> {
    let mut query: Vec<String> = Vec::new();
    query.push("SELECT t.* FROM tags t".to_string());
    query.push("WHERE t.id =".to_string());
    query.push(tag_id.to_string());
    query.push("ORDER BY id ASC".to_string());
    query.push("LIMIT 1".to_string());

    // タグ単体取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let tag = match stmt.query_row([], |row| Tag::by_row(row)) {
        Ok(result) => Some(result),
        Err(_) => None,
    };

    return tag;
}

/** タグ名を基準に、タグを絶対に取得する */
pub fn fetch_tag_by_tag_name(conn: &Connection, tag_name: &String) -> Tag {
    let mut query: Vec<String> = Vec::new();
    query.push("SELECT t.* FROM tags t".to_string());
    query.push("WHERE t.name =".to_string());
    query.push("".to_string() + &"\"" + &tag_name + &"\"");
    query.push("ORDER BY id ASC".to_string());
    query.push("LIMIT 1".to_string());
    println!("{}", &query.join(" "));

    // タグ単体取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let tag = match stmt.query_row([], |row| Tag::by_row(row)) {
        Ok(result) => Some(result),
        Err(_) => Some(create_tag_only_name(conn, tag_name)),
    }
    .unwrap();

    return tag;
}

///

/** タグ名からタグを生成する */
fn create_tag_only_name(conn: &Connection, tag_name: &String) -> Tag {
    // タグ作成
    let now = get_time_of_now();
    let _ = conn.execute(
        "
            INSERT INTO tags (name, created_at, updated_at)
            VALUES (?1, ?2, ?3)
        ",
        params![tag_name, now.clone(), now],
    );

    // 更新したレコードを取得
    let new_report = fetch_tag_by_tag_id(conn, &conn.last_insert_rowid()).unwrap();
    return new_report;
}
