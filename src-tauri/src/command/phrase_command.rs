use rusqlite::params;
use tauri::Window;

use crate::models::Phrase;
use crate::models::PhraseParams;
use crate::query::phrase_query::fetch_phrase_by_phrase_id;
use crate::{fire_tag_changed, get_connection, get_time_of_now};

#[tauri::command]
pub fn phrase_get_all() -> Vec<Phrase> {
    let conn = get_connection();

    let mut query: Vec<String> = Vec::new();
    query.push("SELECT p.* FROM phrase p".to_string());

    // 並び替え
    query.push("ORDER BY priority DESC, id ASC".to_string());

    // フレーズ配列の取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let phrases = stmt
        .query_map([], |row| Phrase::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Phrase>>();

    return phrases;
}

#[tauri::command]
pub fn phrase_update(window: Window, phrase_id: i64, params: PhraseParams) -> Phrase {
    let conn = get_connection();

    // フレーズ更新
    let now = get_time_of_now();
    let _ = &conn.execute(
        "
            UPDATE phrases SET phrase=?1, priority=?2, updated_at=?3
            WHERE id=?6
        ",
        params![params.phrase, params.priority, now, phrase_id],
    );

    // 更新したレコードを取得
    let new_phrase = fetch_phrase_by_phrase_id(&conn, &phrase_id).unwrap();

    // フレーズ更新イベントを発火
    fire_tag_changed(&window);

    return new_phrase;
}
