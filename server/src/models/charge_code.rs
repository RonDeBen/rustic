use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct ChargeCode {
    pub id: i32,
    pub alias: String,
    pub code: String,
    pub is_nc: bool,
}
