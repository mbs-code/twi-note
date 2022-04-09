use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::DB_CONN;

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub id: i64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

pub async fn run_migration() {
    let npool = DB_CONN.get().unwrap();
    let _ = sqlx::migrate!("./migrations").run(npool).await.unwrap();
    println!("migration");
}

#[tauri::command]
pub async fn report_get_all() -> Vec<Report> {
    let pool = DB_CONN.get().unwrap();

    let reports = sqlx::query_as!(
        Report,
        r#"SELECT * from reports
        where deleted_at is null"#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    return reports;
}

#[tauri::command]
pub async fn report_create(title: Option<String>, body: Option<String>) -> Report {
    let pool = DB_CONN.get().unwrap();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let report = sqlx::query_as!(
        Report,
        r#"INSERT INTO reports ( title, body, created_at, updated_at )
        VALUES ( ?, ?, ?, ? )
        RETURNING *"#,
        title,
        body,
        now,
        now,
    )
    .fetch_one(pool)
    .await
    .unwrap();

    return report;
}

#[tauri::command]
pub async fn report_update(id: i64, title: Option<String>, body: Option<String>) -> Report {
    let pool = DB_CONN.get().unwrap();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let report = sqlx::query_as!(
        Report,
        r#"UPDATE reports SET title=?, body=?, updated_at=?
        WHERE id=?
        RETURNING *"#,
        title,
        body,
        now,
        id,
    )
    .fetch_one(pool)
    .await
    .unwrap();

    return report;
}

#[tauri::command]
pub async fn report_delete(id: i64) -> Report {
    let pool = DB_CONN.get().unwrap();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let report = sqlx::query_as!(
        Report,
        r#"UPDATE reports SET updated_at=?, deleted_at=?
        WHERE id=?
        RETURNING *"#,
        now,
        now,
        id,
    )
    .fetch_one(pool)
    .await
    .unwrap();

    return report;
}
