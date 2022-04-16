use super::schema::report_tags;
use super::schema::reports;
use super::schema::tags;

#[derive(Debug, Queryable)]
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

#[derive(Debug, Queryable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

//

#[derive(Debug, Queryable, Associations)]
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

#[derive(Debug)]
pub struct ReportTagRecord {
    pub report: Report,
    pub tags: Vec<Tag>,
}
