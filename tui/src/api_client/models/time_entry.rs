use serde::Deserialize;
use super::day::Day;

#[derive(Deserialize)]
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

