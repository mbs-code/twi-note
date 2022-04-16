use crate::models::{NewTag, Tag};
use crate::schema;
use chrono::Utc;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn convert_tag_name_to_tag(conn: &SqliteConnection, tag_names: Vec<String>) -> Vec<Tag> {
    let mut tags: Vec<Tag> = Vec::new();

    for tag_name in tag_names {
        let tag = fetch_tag_by_tag_name(conn, &tag_name);
        tags.push(tag);
    }

    return tags;
}

fn fetch_tag_by_tag_name(conn: &SqliteConnection, tag_name: &String) -> Tag {
    use crate::schema::tags::dsl::name;
    use crate::schema::tags::dsl::tags;

    // DB のタグを検索する（あれば終わり）
    let db_tag = tags
        .filter(name.eq(tag_name))
        .first::<Tag>(conn)
        .optional()
        .unwrap();
    if let Some(tag) = db_tag {
        return tag;
    }

    // 無ければ新規作成
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let new_tag = NewTag {
        name: tag_name.clone(),
        color: None,
        created_at: now.clone(),
        updated_at: now,
    };

    diesel::insert_into(schema::tags::table)
        .values(&new_tag)
        .execute(conn)
        .unwrap();

    let new_tag = tags
        .find(sql("last_insert_rowid()"))
        .first::<Tag>(conn)
        .unwrap();

    return new_tag;
}