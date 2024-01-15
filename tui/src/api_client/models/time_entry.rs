use super::day::Day;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeEntryVM {
    pub id: i32,
    pub total_time: i64,
    pub note: String,
    pub day: Day,
    pub is_active: bool,
    pub charge_code: Option<ChargeCodeVM>,
    pub start_time: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ChargeCodeVM {
    pub id: i32,
    pub alias: String,
}

impl Eq for TimeEntryVM {}

impl Hash for TimeEntryVM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
