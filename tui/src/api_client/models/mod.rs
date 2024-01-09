use serde::{Deserialize, Serialize};

use crate::components::time_entry::entry::TimeEntry;

use self::{charge_code::ChargeCode, day::Day, time_entry::TimeEntryVM};
use std::collections::HashMap;

pub mod charge_code;
pub mod day;
pub mod time_entry;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct FullState {
    pub time_entries: HashMap<Day, Vec<TimeEntryVM>>,
    pub charge_codes: Vec<ChargeCode>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct DayTimeEntries {
    pub day: Day,
    pub entries: Vec<TimeEntryVM>,
}

impl FullState {
    pub fn get_charge_code_names(&self) -> Vec<String> {
        self.charge_codes
            .iter()
            .map(|cc| cc.alias.clone())
            .collect()
    }

    pub fn get_time_entries_for_day(&self, day: Day) -> Vec<TimeEntry> {
        match self.time_entries.get(&day) {
            Some(entries) => entries.iter().map(|x| x.into()).collect(),
            None => Vec::default(),
        }
    }
}
