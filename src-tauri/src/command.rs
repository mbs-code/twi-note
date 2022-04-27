use rusqlite::params;

use crate::models::{Report, ReportWithTagParams, TagParams};
use crate::models::{ReportWithTag, Tag};
use crate::query::tag_query::fetch_tag_by_tag_id;
use crate::query::{
    report_query::fetch_report_with_tag_by_report_id, report_tag_query::associate_report_tag,
    tag_query::fetch_tags_by_report_id,
};
use crate::{get_connection, get_time_of_now};

#[tauri::command]
pub fn report_get_all(
    tag_name: Option<String>,
    page: i32,
    count: i32,
    latest: bool,
) -> Vec<ReportWithTag> {
    let conn = get_connection();

    let mut query: Vec<String> = Vec::new();
    query.push("SELECT r.* FROM reports r".to_string());
    query.push("LEFT JOIN report_tags rt ON r.id = rt.report_id".to_string());
    query.push("LEFT JOIN tags t ON rt.tag_id = t.id".to_string());

    // タグ抽出
    if let Some(name) = tag_name {
        // joinしてタグ名で抽出
        query.push("WHERE t.name =".to_string());
        query.push("".to_string() + &"\"" + &name + &"\"");
    }

    // 並び替え
    let order = if latest { "DESC" } else { "ASC" };
    query.push("ORDER BY id".to_string());
    query.push(order.to_string());

    // ページネーション
    query.push("LIMIT".to_string());
    query.push(count.to_string());
    query.push("OFFSET".to_string());
    query.push(((page - 1) * count).to_string());

    // レポート配列の取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let reports = stmt
        .query_map([], |row| Report::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Report>>();

    // 各レポートにタグを紐づける
    let report_with_tags = reports
        .into_iter()
        .map(|report| {
            let tags = fetch_tags_by_report_id(&conn, &report.id);
            return ReportWithTag::new(report, tags);
        })
        .collect::<Vec<ReportWithTag>>();

    return report_with_tags;
}

#[tauri::command]
pub fn report_create(params: ReportWithTagParams) -> ReportWithTag {
    let conn = get_connection();

    // レポート作成
    let now = get_time_of_now();
    let _ = &conn.execute(
        "
            INSERT INTO reports (title, body, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4)
        ",
        params![params.title, params.body, now.clone(), now],
    );

    // タグのバインド
    let report_id = &conn.last_insert_rowid();
    associate_report_tag(&conn, &report_id, &params.tag_names);

    // 更新したレコードを取得
    let new_report = fetch_report_with_tag_by_report_id(&conn, &report_id);
    return new_report;
}

#[tauri::command]
pub fn report_update(report_id: i64, params: ReportWithTagParams) -> ReportWithTag {
    let conn = get_connection();

    // レポート更新
    let now = get_time_of_now();
    let _ = &conn.execute(
        "
            UPDATE reports SET title=?1, body=?2, updated_at=?3
            WHERE id=?4
        ",
        params![params.title, params.body, now, report_id],
    );

    // タグのバインド
    associate_report_tag(&conn, &report_id, &params.tag_names);

    // 更新したレコードを取得
    let new_report = fetch_report_with_tag_by_report_id(&conn, &report_id);
    return new_report;
}

#[tauri::command]
pub fn report_remove(report_id: i32) -> bool {
    let conn = get_connection();

    // レポート削除
    let now = get_time_of_now();
    let _ = &conn.execute(
        "
            UPDATE reports SET deleted_at=?1
            WHERE id=?2
        ",
        params![now, report_id],
    );

    return true;
}

/// ////////////////////////////////////////////////////////////

#[tauri::command]
pub fn tag_get_all(has_pinned: bool) -> Vec<Tag> {
    let conn = get_connection();

    let mut query: Vec<String> = Vec::new();
    query.push("SELECT t.* FROM tags t".to_string());

    // タグ抽出
    if has_pinned {
        // joinしてタグ名で抽出
        query.push("WHERE t.is_pinned = 1".to_string());
    }

    // 並び替え
    query.push("ORDER BY priority DESC, id ASC".to_string());

    // タグ配列の取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let tags = stmt
        .query_map([], |row| Tag::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Tag>>();

    return tags;
}

#[tauri::command]
pub fn tag_update(tag_id: i64, params: TagParams) -> Tag {
    let conn = get_connection();

    // タグ更新
    let is_pinned = if params.is_pinned { 1 } else { 0 };
    let now = get_time_of_now();
    let _ = &conn.execute(
        "
            UPDATE tags SET name=?1, color=?2, is_pinned=?3, priority=?4, updated_at=?5
            WHERE id=?6
        ",
        params![
            params.name,
            params.color,
            is_pinned,
            params.priority,
            now,
            tag_id,
        ],
    );

    // 更新したレコードを取得
    let new_tag = fetch_tag_by_tag_id(&conn, &tag_id).unwrap();
    return new_tag;
}
