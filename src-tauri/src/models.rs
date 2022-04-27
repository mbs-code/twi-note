use rusqlite::{Error, Row};

#[derive(Debug)]
pub struct Report {
    pub id: i64,
    pub title: Option<String>,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
impl Report {
    pub fn by_row(row: &Row) -> Result<Report, Error> {
        Ok(Report {
            id: row.get(0)?,
            title: row.get(1)?,
            body: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            deleted_at: row.get(5)?,
        })
    }
}

#[derive(Debug)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub is_pinned: i64,
    pub priority: i64,
    pub created_at: String,
    pub updated_at: String,
}
impl Tag {
    pub fn by_row(row: &Row) -> Result<Tag, Error> {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            is_pinned: row.get(3)?,
            priority: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }
}

#[derive(Debug)]
pub struct TagParams {
    pub name: String,
    pub color: Option<String>,
    pub is_pinned: bool,
    pub priority: i64,
}

#[derive(Debug)]
pub struct ReportTag {
    pub id: i64,
    pub report_id: i64,
    pub tag_id: i64,
    pub created_at: String,
}

/// ////////////////////////////////////////

#[derive(Debug)]
pub struct ReportWithTagParams {
    pub title: Option<String>,
    pub body: String,
    pub tag_names: Vec<String>,
}

#[derive(Debug)]
pub struct ReportWithTag {
    pub id: i64,
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

/// ////////////////////////////////////////

pub struct ReportTagJoinTag {
    pub id: i64,
    pub report_id: i64,
    pub tag_id: i64,
    pub created_at: String,
    pub tag_name: String,
}
impl ReportTagJoinTag {
    pub fn by_row(row: &Row) -> Result<ReportTagJoinTag, Error> {
        Ok(ReportTagJoinTag {
            id: row.get(0)?,
            report_id: row.get(1)?,
            tag_id: row.get(2)?,
            created_at: row.get(3)?,
            tag_name: row.get(4)?,
        })
    }
}
