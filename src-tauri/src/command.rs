// use chrono::Utc;
// use serde::{Deserialize, Serialize};

// use crate::DB_CONN;

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// pub struct Report {
//     pub id: i64,
//     pub title: Option<String>,
//     pub body: Option<String>,
//     pub created_at: String,
//     pub updated_at: String,
//     pub deleted_at: Option<String>,
// }

// pub async fn run_migration() {
//     let npool = DB_CONN.get().unwrap();
//     let _ = sqlx::migrate!("./migrations").run(npool).await.unwrap();
//     println!("migration");
// }

// #[tauri::command]
// pub async fn report_get_all(page: i64, count: i64, latest: bool) -> Vec<Report> {
//     let pool = DB_CONN.get().unwrap();

//     // NOTE: dynamic query は型付きバインドできないため、手動で作成する。
//     // インジェクションに注意
//     let mut query = String::new();
//     query.push_str("SELECT * FROM reports WHERE deleted_at IS NULL ORDER BY id ");
//     query.push_str(if latest { "DESC" } else { "ASC" });
//     query.push_str(" LIMIT ");
//     query.push_str(&count.to_string());
//     query.push_str(" OFFSET ");
//     query.push_str(&((page - 1) * count).to_string());
//     let reports = sqlx::query_as::<_, Report>(&query)
//         .fetch_all(pool)
//         .await
//         .unwrap();

//     return reports;
// }

// #[tauri::command]
// pub async fn report_create(title: Option<String>, body: Option<String>) -> Report {
//     let pool = DB_CONN.get().unwrap();
//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

//     let report = sqlx::query_as!(
//         Report,
//         r#"INSERT INTO reports ( title, body, created_at, updated_at )
//         VALUES ( ?, ?, ?, ? )
//         RETURNING *"#,
//         title,
//         body,
//         now,
//         now,
//     )
//     .fetch_one(pool)
//     .await
//     .unwrap();

//     return report;
// }

// #[tauri::command]
// pub async fn report_update(id: i64, title: Option<String>, body: Option<String>) -> Report {
//     let pool = DB_CONN.get().unwrap();
//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

//     let report = sqlx::query_as!(
//         Report,
//         r#"UPDATE reports SET title=?, body=?, updated_at=?
//         WHERE id=?
//         RETURNING *"#,
//         title,
//         body,
//         now,
//         id,
//     )
//     .fetch_one(pool)
//     .await
//     .unwrap();

//     return report;
// }

// #[tauri::command]
// pub async fn report_remove(id: i64) -> Report {
//     let pool = DB_CONN.get().unwrap();
//     let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

//     let report = sqlx::query_as!(
//         Report,
//         r#"UPDATE reports SET updated_at=?, deleted_at=?
//         WHERE id=?
//         RETURNING *"#,
//         now,
//         now,
//         id,
//     )
//     .fetch_one(pool)
//     .await
//     .unwrap();

//     return report;
// }
