use rusqlite::params;
use tauri::Window;

use crate::fire_phrase_changed;
use crate::models::Phrase;
use crate::models::PhraseParams;
use crate::query::phrase_query::fetch_phrase_by_phrase_id;
use crate::{get_connection, get_time_of_now};

#[tauri::command]
pub fn phrase_get_all() -> Vec<Phrase> {
    let conn = get_connection();

    let mut query: Vec<String> = Vec::new();
    query.push("SELECT p.* FROM phrases p".to_string());

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
pub fn phrase_create(window: Window, params: PhraseParams) -> Phrase {
    let conn = get_connection();

    // フレーズ作成
    let now = get_time_of_now();
    let _ = &conn.execute(
        "
            INSERT INTO phrases (text, color, priority, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        params![params.text, params.color, params.priority, now.clone(), now],
    );

    // 作成したレコードを取得
    let phrase_id = &conn.last_insert_rowid();
    let new_phrase = fetch_phrase_by_phrase_id(&conn, &phrase_id).unwrap();

    // フレーズ更新イベントを発火
    fire_phrase_changed(&window);

    return new_phrase;
}

#[tauri::command]
pub fn phrase_update(window: Window, phrase_id: i64, params: PhraseParams) -> Phrase {
    let conn = get_connection();

    // フレーズ更新
    let now = get_time_of_now();
    let _ = &conn.execute(
        "
            UPDATE phrases SET text=?1, color=?2, priority=?3, updated_at=?4
            WHERE id=?5
        ",
        params![params.text, params.color, params.priority, now, phrase_id],
    );

    // 更新したレコードを取得
    let new_phrase = fetch_phrase_by_phrase_id(&conn, &phrase_id).unwrap();

    // フレーズ更新イベントを発火
    fire_phrase_changed(&window);

    return new_phrase;
}

#[tauri::command]
pub fn phrase_remove(window: Window, phrase_id: i64) -> bool {
    let conn = get_connection();

    // フレーズ削除
    let _ = &conn.execute(
        "
            DELETE FROM phrases WHERE id=?1
        ",
        params![phrase_id],
    );

    // フレーズ更新イベントを発火
    fire_phrase_changed(&window);

    return true;
}
