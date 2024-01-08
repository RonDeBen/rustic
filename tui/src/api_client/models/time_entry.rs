use super::day::Day;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeEntryVM {
    // start_time shouldn't be needed by the TUI
    pub id: i32,
    pub total_time: f64,
    pub note: String,
    pub day: Day,
    pub is_active: bool,
    pub charge_code: Option<String>,
}

impl Eq for TimeEntryVM {}

impl Hash for TimeEntryVM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
