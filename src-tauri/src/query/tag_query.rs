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

// pub fn convert_tag_name_to_tag(conn: &mut SqliteConnection, tag_names: Vec<String>) -> Vec<Tag> {
//     let mut tags: Vec<Tag> = Vec::new();

//     for tag_name in tag_names {
//         let tag = fetch_tag_by_tag_name(conn, &tag_name);
//         tags.push(tag);
//     }

//     return tags;
// }

// pub fn convert_report_tag_to_tag(
//     conn: &mut SqliteConnection,
//     report_tags: Vec<ReportTag>,
// ) -> Vec<Tag> {
//     let mut tags: Vec<Tag> = Vec::new();

//     for report_tag in report_tags {
//         let tag = fetch_tag_by_id(conn, &report_tag.tag_id);
//         tags.push(tag);
//     }

//     return tags;
// }

// pub fn update_tag_returning(
//     conn: &mut SqliteConnection,
//     tag_id: &i32,
//     name_val: String,
//     color_val: Option<String>,
//     is_pinned_val: i32,
//     priority_val: i32,
// ) -> Tag {
//     use crate::schema::tags::dsl::color;
//     use crate::schema::tags::dsl::is_pinned;
//     use crate::schema::tags::dsl::name;
//     use crate::schema::tags::dsl::priority;
//     use crate::schema::tags::dsl::tags;
//     use crate::schema::tags::dsl::updated_at;

//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

//     // FIXME: 何故か self が要求される
//     diesel::update(tags::find(tags, tag_id))
//         .set((
//             name.eq(name_val),
//             color.eq(color_val),
//             is_pinned.eq(is_pinned_val),
//             priority.eq(priority_val),
//             updated_at.eq(now),
//         ))
//         .execute(conn)
//         .unwrap();

//     let tag = tags.find(tag_id).first::<Tag>(conn).unwrap();
//     return tag;
// }

// ///

// pub fn fetch_tag_by_id(conn: &mut SqliteConnection, tag_id: &i32) -> Tag {
//     use crate::schema::tags::dsl::tags;

//     let new_tag = tags.find(tag_id).first::<Tag>(conn).unwrap();
//     return new_tag;
// }

// pub fn fetch_tag_by_tag_name(conn: &mut SqliteConnection, tag_name: &String) -> Tag {
//     use crate::schema::tags::dsl::name;
//     use crate::schema::tags::dsl::tags;

//     // DB のタグを検索する（あれば終わり）
//     let db_tag = tags
//         .filter(name.eq(tag_name))
//         .first::<Tag>(conn)
//         .optional()
//         .unwrap();
//     if let Some(tag) = db_tag {
//         return tag;
//     }

//     // 無ければ新規作成
//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
//     let new_tag = NewTag {
//         name: tag_name.clone(),
//         color: None,
//         is_pinned: None,
//         priority: None,
//         created_at: now.clone(),
//         updated_at: now,
//     };

//     diesel::insert_into(schema::tags::table)
//         .values(&new_tag)
//         .execute(conn)
//         .unwrap();

//     let new_tag = tags
//         .find(sql("last_insert_rowid()"))
//         .first::<Tag>(conn)
//         .unwrap();

//     return new_tag;
// }
