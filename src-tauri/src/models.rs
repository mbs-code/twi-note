use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TagParams {
    pub name: String,
    pub color: Option<String>,
    pub has_pinned: bool,
    pub priority: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportTag {
    pub id: i64,
    pub report_id: i64,
    pub tag_id: i64,
    pub created_at: String,
}

/// ////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportWithTagParams {
    pub title: Option<String>,
    pub body: String,
    pub tag_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

/// ////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct Phrase {
    pub id: i64,
    pub text: String,
    pub color: Option<String>,
    pub priority: i64,
    pub created_at: String,
    pub updated_at: String,
}
impl Phrase {
    pub fn by_row(row: &Row) -> Result<Phrase, Error> {
        Ok(Phrase {
            id: row.get(0)?,
            text: row.get(1)?,
            color: row.get(2)?,
            priority: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhraseParams {
    pub text: String,
    pub color: Option<String>,
    pub priority: i64,
}
