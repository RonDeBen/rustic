use self::{
    charge_code::ChargeCode,
    time_entry::{Day, TimeEntryVM},
};
use std::collections::HashMap;

pub mod charge_code;
pub mod time_entry;

#[derive(serde::Serialize)]
pub struct FullState {
    pub time_entries: HashMap<Day, Vec<TimeEntryVM>>,
    pub charge_codes: Vec<ChargeCode>,
}

#[derive(serde::Serialize)]
pub struct DayTimeEntries {
    pub day: Day,
    pub entries: Vec<TimeEntryVM>,
}

