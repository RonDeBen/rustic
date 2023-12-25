use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct TimeEntry {
    pub id: i32,
    pub start_time: i64,
    pub total_time: f64,
    pub note: String,
    pub day: i32,
}
