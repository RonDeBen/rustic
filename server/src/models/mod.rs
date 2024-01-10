use self::{
    charge_code::ChargeCode,
    time_entry::{Day, TimeEntryVM, TimeEntryRaw},
};
use std::collections::HashMap;

pub mod charge_code;
pub mod time_entry;

#[derive(serde::Serialize)]
pub struct FullState {
    pub time_entries: HashMap<Day, Vec<TimeEntryVM>>,
    pub charge_codes: Vec<ChargeCode>,
}

#[derive(serde::Serialize, Debug)]
pub struct DayTimeEntries {
    pub day: Day,
    pub entries: Vec<TimeEntryVM>,
}

impl DayTimeEntries {
    pub fn new(day: Day, entries: &[TimeEntryRaw]) -> Self {
        let mut vms: Vec<TimeEntryVM> = entries.iter().map(|x| x.into()).collect();
        vms.sort_by(|a, b| a.id.cmp(&b.id));
        Self { day, entries: vms }
    }
}
