#[derive(Debug)]
pub struct Report {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug)]
pub struct ReportWithTag {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub tags: Vec<Tag>,
}
impl ReportWithTag {
    pub fn new(report: Report, tags: Vec<Tag>) -> Self {
        ReportWithTag {
            id: report.id,
            title: report.title,
            body: report.body,
            created_at: report.created_at,
            updated_at: report.updated_at,
            deleted_at: report.deleted_at,
            tags,
        }
    }
}

///

#[derive(Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub is_pinned: i32,
    pub priority: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct ReportTag {
    pub id: i32,
    pub report_id: i32,
    pub tag_id: i32,
    pub created_at: String,
}
