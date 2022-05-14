use rusqlite::params;
use tauri::Window;

use crate::models::Tag;
use crate::models::TagParams;
use crate::query::tag_query::fetch_tag_by_tag_id;
use crate::{fire_tag_changed, get_connection, get_time_of_now};

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
    println!("{:?}", &query.join(" "));
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let tags = stmt
        .query_map([], |row| Tag::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Tag>>();

    return tags;
}

#[tauri::command]
pub fn tag_update(window: Window, tag_id: i64, params: TagParams) -> Tag {
    let conn = get_connection();

    // タグ更新
    let is_pinned = if params.has_pinned { 1 } else { 0 };
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

    // タグ更新イベントを発火
    fire_tag_changed(&window);

    return new_tag;
}
