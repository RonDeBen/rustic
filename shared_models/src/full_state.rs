use serde::Serialize;
use std::collections::HashMap;

use crate::{charge_code::ChargeCode, day::Day, time_entry::TimeEntryVM};

#[derive(Serialize)]
pub struct FullState {
    pub time_entries: HashMap<Day, Vec<TimeEntryVM>>,
    pub charge_codes: Vec<ChargeCode>,
}

#[derive(Debug)]
pub struct DayTimeEntries {
    pub day: Day,
    pub entries: Vec<TimeEntryVM>,
}
