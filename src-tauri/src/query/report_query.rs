use crate::models::{NewReport, Report};
use crate::schema;
use chrono::Utc;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn craete_report_returning(
    conn: &SqliteConnection,
    title_val: Option<String>,
    body_val: String,
) -> Report {
    use crate::schema::reports::dsl::reports;

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let new_report = NewReport {
        title: title_val,
        body: body_val,
        created_at: now.clone(),
        updated_at: now,
    };

    diesel::insert_into(schema::reports::table)
        .values(&new_report)
        .execute(conn)
        .expect("Error saving new post");

    let report = reports
        .find(sql("last_insert_rowid()"))
        .first::<Report>(conn)
        .unwrap();
    return report;
}

pub fn update_report_returning(
    conn: &SqliteConnection,
    rid: i32,
    title_val: Option<String>,
    body_val: String,
) -> Report {
    use crate::schema::reports::dsl::body;
    use crate::schema::reports::dsl::reports;
    use crate::schema::reports::dsl::title;
    use crate::schema::reports::dsl::updated_at;

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    diesel::update(reports.find(rid))
        .set((title.eq(title_val), body.eq(body_val), updated_at.eq(now)))
        .execute(conn)
        .unwrap();

    let report = reports.find(rid).first::<Report>(conn).unwrap();
    return report;
}
