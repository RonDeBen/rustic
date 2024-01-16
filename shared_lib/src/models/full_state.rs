use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{charge_code::ChargeCode, day::Day, time_entry::TimeEntryVM};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullState {
    pub time_entries: HashMap<Day, Vec<TimeEntryVM>>,
    pub charge_codes: Vec<ChargeCode>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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

    pub fn get_vms_for_day(&self, day: Day) -> Option<&Vec<TimeEntryVM>> {
        self.time_entries.get(&day)
    }
}
