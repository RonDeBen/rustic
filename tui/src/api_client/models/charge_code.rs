
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChargeCode {
    pub id: i32,
    pub alias: String,
    pub code: String,
    pub is_nc: bool,
}

