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
