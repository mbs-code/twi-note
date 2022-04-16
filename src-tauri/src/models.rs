#[derive(Queryable)]
pub struct Report {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
