use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ChargeCode {
    pub id: i32,
    pub alias: String,
    pub code: String,
    pub is_nc: bool,
}
