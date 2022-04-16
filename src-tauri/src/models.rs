use super::schema::reports;

#[derive(Queryable, Debug)]
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
