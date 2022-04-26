// pub fn associate_report_tag(conn: &mut SqliteConnection, report: &Report, tags: &Vec<Tag>) {
//     // レポートに紐づいているタグを取得する
//     let report_tags = fetch_report_tag_by_report_id(conn, report.id);

//     // タグを回して、紐づいていないものを追加
//     for tag in tags {
//         let has = report_tags
//             .iter()
//             .filter(|&report_tag| tag.id.eq(&report_tag.tag_id))
//             .count();

//         // 一つも無いなら追加
//         if has == 0 {
//             attach_report_tag(conn, &report, &tag);
//         }
//     }

//     // 逆に紐づきを回して、タグに無いものを削除
//     for report_tag in report_tags {
//         let has = tags
//             .iter()
//             .filter(|&tag| tag.id.eq(&report_tag.tag_id))
//             .count();

//         // 一つも無いなら削除
//         if has == 0 {
//             detach_report_tag(conn, &report_tag);
//         }
//     }
// }

// pub fn fetch_report_tag_by_report_id(conn: &mut SqliteConnection, rid: i32) -> Vec<ReportTag> {
//     use crate::schema::report_tags::dsl::report_id;
//     use crate::schema::report_tags::dsl::report_tags;

//     let db_report_tags = report_tags
//         .filter(report_id.eq(rid))
//         .load::<ReportTag>(conn)
//         .unwrap();

//     return db_report_tags;
// }

// pub fn fetch_report_tag_by_tag_id(conn: &mut SqliteConnection, tid: i32) -> Vec<ReportTag> {
//     use crate::schema::report_tags::dsl::report_tags;
//     use crate::schema::report_tags::dsl::tag_id;

//     let db_report_tags = report_tags
//         .filter(tag_id.eq(tid))
//         .load::<ReportTag>(conn)
//         .unwrap();

//     return db_report_tags;
// }

// ///

// fn attach_report_tag(conn: &mut SqliteConnection, report: &Report, tag: &Tag) {
//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
//     let new_report_tag = NewReportTag {
//         report_id: report.id,
//         tag_id: tag.id,
//         created_at: now,
//     };

//     diesel::insert_into(schema::report_tags::table)
//         .values(&new_report_tag)
//         .execute(conn)
//         .unwrap();
// }

// fn detach_report_tag(conn: &mut SqliteConnection, report_tag: &ReportTag) {
//     use crate::schema::report_tags::dsl::report_tags;

//     diesel::delete(report_tags.find(report_tag.id))
//         .execute(conn)
//         .unwrap();
// }
