use Chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct log {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub window: String,
    pub focus: String
}