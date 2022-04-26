pub struct Report {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub is_pinned: i32,
    pub priority: i32,
    pub created_at: String,
    pub updated_at: String,
}

pub struct ReportTag {
    pub id: i32,
    pub report_id: i32,
    pub tag_id: i32,
    pub created_at: String,
}
