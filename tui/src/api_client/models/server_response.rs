use super::{charge_code::ChargeCode, time_entry::TimeEntry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct ServerResponse {
    weekly_entries: WeeklyTimeEntries,
    charge_codes: Vec<ChargeCode>,
}

#[derive(Serialize, Deserialize)]
struct WeeklyTimeEntries {
    entries: HashMap<String, Vec<TimeEntry>>,
}
