#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use chrono::Utc;
use diesel::sqlite::SqliteConnection;
use diesel::{dsl::sql, prelude::*};
use dotenv::dotenv;
use std::env;

use crate::models::{NewReport, Report};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_report(conn: &SqliteConnection, title: Option<String>, body: String) -> Report {
    use crate::schema::reports::dsl;

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let new_report = NewReport {
        title,
        body,
        created_at: now.clone(),
        updated_at: now,
    };

    diesel::insert_into(schema::reports::table)
        .values(&new_report)
        .execute(conn)
        .expect("Error saving new post");

    let result = dsl::reports
        .find(sql("last_insert_rowid()"))
        .first::<Report>(conn)
        .unwrap();
    return result;
}

pub fn update_report(
    conn: &SqliteConnection,
    id: i32,
    title: Option<String>,
    body: String,
) -> Report {
    use crate::schema::reports::dsl;

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    diesel::update(dsl::reports.find(id))
        .set((
            dsl::title.eq(title),
            dsl::body.eq(body),
            dsl::updated_at.eq(now),
        ))
        .execute(conn)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    let result = dsl::reports
        .find(sql("last_insert_rowid()"))
        .first::<Report>(conn)
        .unwrap();
    return result;
}
