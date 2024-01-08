use super::day::Day;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Debug)]
pub struct TimeEntryVM {
    // start_time shouldn't be needed by the TUI
    pub id: i32,
    pub total_time: i64,
    pub note: String,
    pub day: Day,
    //TODO:
    pub is_active: bool,
    pub charge_code: String,
}
