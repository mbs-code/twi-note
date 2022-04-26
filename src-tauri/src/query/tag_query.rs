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
