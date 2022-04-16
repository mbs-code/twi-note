table! {
    report_tag (id) {
        id -> Integer,
        report_id -> Integer,
        tag_id -> Integer,
        created_at -> Text,
    }
}

table! {
    reports (id) {
        id -> Integer,
        title -> Nullable<Text>,
        body -> Text,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        color -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    report_tag,
    reports,
    tags,
);
