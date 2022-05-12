use rusqlite::Connection;

use crate::models::Phrase;

/** フレーズIDを基準に、フレーズを取得する */
pub fn fetch_phrase_by_phrase_id(conn: &Connection, phrase_id: &i64) -> Option<Phrase> {
    let mut query: Vec<String> = Vec::new();
    query.push("SELECT p.* FROM phrases p".to_string());
    query.push("WHERE p.id =".to_string());
    query.push(phrase_id.to_string());
    query.push("ORDER BY id ASC".to_string());
    query.push("LIMIT 1".to_string());

    // フレーズ単体取得
    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let phrase = match stmt.query_row([], |row| Phrase::by_row(row)) {
        Ok(result) => Some(result),
        Err(_) => None,
    };

    return phrase;
}
