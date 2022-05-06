use rusqlite::params;

use crate::models::ReportWithTag;
use crate::models::{Report, ReportWithTagParams};
use crate::query::{
    report_query::fetch_report_with_tag_by_report_id, report_tag_query::associate_report_tag,
    tag_query::fetch_tags_by_report_id,
};
use crate::{get_connection, get_time_of_now};
use chrono::{Duration, NaiveDate, NaiveDateTime};

#[tauri::command]
pub fn report_get_all(
    text: Option<String>,
    page: i32,
    count: i32,
    latest: bool,
    use_updated_at: bool,
) -> Vec<ReportWithTag> {
    let conn = get_connection();

    // 検索に使う日付カラム名
    let timestamp_column = if use_updated_at {
        "r.updated_at"
    } else {
        "r.created_at"
    };

    // クエリ作成
    let mut query: Vec<String> = Vec::new();
    query.push("SELECT r.* FROM reports r".to_string());

    // deleted_at があるものを除外
    query.push("WHERE r.deleted_at IS NULL".to_string());

    // テキスト抽出
    if let Some(text) = text {
        // NOTE:
        // - スペースごとに処理
        // - `before:` `after:` があったら日付検索
        //   - フォーマットは `yyyy-mm-dd`
        //   - タイムゾーンを考慮して一日の範囲を取得する
        // - `tag:` が先頭にあったらタグ一致検索
        // - 何もなければ一部でも含まれているものを検索

        // スペースで分割し、それぞれで処理
        let words = text.split([' ', '　']).collect::<Vec<&str>>();
        for word in words {
            let qword = word.trim();

            if qword.starts_with("before:") {
                // この日以前のものを取得する
                let trim_word = qword.trim_start_matches("before:");
                if trim_word.len() == 10 {
                    // タイムゾーンを考慮して時刻換算する
                    let date = NaiveDate::parse_from_str(trim_word, "%Y-%m-%d")
                        .unwrap()
                        .and_hms(0, 0, 0);
                    let before: NaiveDateTime =
                        date - Duration::hours(9) + Duration::days(1) - Duration::seconds(1);
                    let before_str = before.format("%Y-%m-%d %H:%M:%S").to_string();

                    let keyword = "\'".to_string() + &before_str + "\'";
                    query.push("AND".to_string());
                    query.push(timestamp_column.to_string());
                    query.push("<=".to_string());
                    query.push(keyword);
                }
            } else if qword.starts_with("after:") {
                // この日以降のものを取得する
                let trim_word = qword.trim_start_matches("after:");
                if trim_word.len() == 10 {
                    // タイムゾーンを考慮して時刻換算する
                    let date = NaiveDate::parse_from_str(trim_word, "%Y-%m-%d")
                        .unwrap()
                        .and_hms(0, 0, 0);
                    let after: NaiveDateTime = date - Duration::hours(9);
                    let after_str = after.format("%Y-%m-%d %H:%M:%S").to_string();

                    let keyword = "\'".to_string() + &after_str + "\'";
                    query.push("AND".to_string());
                    query.push(timestamp_column.to_string());
                    query.push(">=".to_string());
                    query.push(keyword);
                }
            } else if qword.starts_with("tag:") {
                // タグ検索要素なら、タグの完全一致を行う（サブクエリ）
                let trim_word = qword.trim_start_matches("tag:");
                if trim_word.len() > 0 {
                    let keyword = "\'".to_string() + trim_word + "\'";
                    query.push("AND r.id in (".to_string());
                    query.push("SELECT rt.report_id FROM report_tags rt".to_string());
                    query.push("LEFT JOIN tags t ON rt.tag_id = t.id".to_string());
                    query.push("WHERE rt.report_id = r.id".to_string());
                    query.push("AND t.name LIKE".to_string());
                    query.push(keyword);
                    query.push(")".to_string());
                }
            } else if qword.len() > 0 {
                // title, body, tag_name の like 検索（一括り）
                let keyword = "\'%".to_string() + qword + "%\'";
                query.push("AND (r.body LIKE".to_string());
                query.push(keyword.to_string());
                query.push("OR r.title LIKE".to_string());
                query.push(keyword.to_string());

                query.push("OR r.id in (".to_string());
                query.push("SELECT rt.report_id FROM report_tags rt".to_string());
                query.push("LEFT JOIN tags t ON rt.tag_id = t.id".to_string());
                query.push("WHERE rt.report_id = r.id".to_string());
                query.push("AND t.name LIKE".to_string());
                query.push(keyword);
                query.push("))".to_string());
            }
        }
    }

    // 並び替え
    let order = if latest { "DESC" } else { "ASC" };
    query.push("ORDER BY".to_string());
    query.push(timestamp_column.to_string());
    query.push(order.to_string());
    query.push(", r.id".to_string());
    query.push(order.to_string());

    // ページネーション
    query.push("LIMIT".to_string());
    query.push(count.to_string());
    query.push("OFFSET".to_string());
    query.push(((page - 1) * count).to_string());
    println!("{:?}", &query.join(" "));

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
