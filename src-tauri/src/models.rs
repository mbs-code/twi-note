use super::schema::report_tags;
use super::schema::reports;
use super::schema::tags;
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Debug, Queryable, Serialize, Deserialize, QueryableByName, FromSqlRow)]
pub struct Report {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Insertable)]
#[table_name = "reports"]
pub struct NewReport {
    pub title: Option<String>,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

//

#[derive(Identifiable, Debug, Queryable, Serialize, Deserialize)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub is_pinned: i32,
    pub priority: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub color: Option<String>,
    pub is_pinned: Option<i32>,
    pub priority: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

//

#[derive(Identifiable, Debug, Queryable, Associations)]
#[belongs_to(Report)]
#[belongs_to(Tag)]
pub struct ReportTag {
    pub id: i32,
    pub report_id: i32,
    pub tag_id: i32,
    pub created_at: String,
}

#[derive(Insertable)]
#[table_name = "report_tags"]
pub struct NewReportTag {
    pub report_id: i32,
    pub tag_id: i32,
    pub created_at: String,
}

///

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportWithTag {
    pub report: Report,
    pub tags: Vec<Tag>,
}
