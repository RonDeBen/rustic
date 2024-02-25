use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Debug, sqlx::FromRow)]
pub struct ChargeCode {
    pub id: i32,
    pub alias: String,
    pub code: String,
    pub is_nc: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct ChargeCodeVM {
    pub id: i32,
    pub alias: String,
}


