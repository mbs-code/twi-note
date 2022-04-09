use std::path::Path;

use chrono::Local;
use sqlx::migrate::Migrator;

use crate::DB_CONN;

#[derive(Debug)]
pub struct Report {
    pub id: i64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

pub async fn run_migration() {
    let m = Migrator::new(Path::new("./migrations")).await.unwrap();

    let npool = DB_CONN.get().unwrap();
    let _ = m.run(npool).await.unwrap();
}

#[tauri::command]
pub async fn report_create(title: String, body: String) -> Report {
    let pool = DB_CONN.get().unwrap();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let record = sqlx::query_as!(
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

    return record;
}
