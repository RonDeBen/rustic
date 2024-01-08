use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Debug)]
pub struct ChargeCode {
    pub id: i32,
    pub alias: String,
    pub code: String,
    pub is_nc: bool,
}
