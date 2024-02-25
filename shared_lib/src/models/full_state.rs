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

#[derive(Debug, Clone)]
pub struct TimeEntriesDiff {
    pub to_upsert: Vec<TimeEntryVM>,
    pub to_delete: Vec<i32>, // Assuming `id` is sufficient to identify deletions
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

    pub fn diff(&self, other: &Self) -> TimeEntriesDiff {
        let mut to_upsert = Vec::new();
        let mut self_entries_map = HashMap::new();

        // Build a map for quick lookup of self entries
        for entries in self.time_entries.values().flatten() {
            self_entries_map.insert(entries.id, entries);
        }

        // Entries in `other` to check against `self` for changes
        for (&day, entries) in &other.time_entries {
            if let Some(self_entries) = self.time_entries.get(&day) {
                let self_entries_map: HashMap<i32, &TimeEntryVM> =
                    self_entries.iter().map(|e| (e.id, e)).collect();

                for entry in entries {
                    if let Some(self_entry) = self_entries_map.get(&entry.id) {
                        if entry != *self_entry {
                            // Entry has changed, mark for upsert from `other`
                            to_upsert.push(entry.clone());
                        }
                    } else {
                        // Entry not found in `self`, it's a new entry, mark for upsert
                        to_upsert.push(entry.clone());
                    }
                }
            } else {
                // If the day doesn't exist in `self`, all these entries are new or changed
                to_upsert.extend(entries.clone());
            }
        }

        // Entries in `self` that aren't in `other`, mark for deletion
        let to_delete: Vec<i32> = self_entries_map
            .keys()
            .filter(|id| !other.time_entries.values().flatten().any(|e| e.id == **id))
            .cloned()
            .collect();

        TimeEntriesDiff { to_upsert, to_delete }
    }
}
