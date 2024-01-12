use super::day::Day;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeEntryVM {
    pub id: i32,
    pub total_time: f64,
    pub note: String,
    pub day: Day,
    pub is_active: bool,
    pub charge_code: Option<String>,
    pub start_time: Option<NaiveDateTime>,
}

impl Eq for TimeEntryVM {}

impl Hash for TimeEntryVM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
