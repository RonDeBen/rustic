use shared_models::{day::Day, full_state::FullState};

use crate::components::time_entry::entry::TimeEntry;

pub trait FullStateExt {
    fn get_time_entries_for_day(&self, day: Day) -> Vec<TimeEntry>;
}

impl FullStateExt for FullState {
    fn get_time_entries_for_day(&self, day: Day) -> Vec<TimeEntry> {
        match self.time_entries.get(&day) {
            Some(entries) => entries.iter().map(|x| x.into()).collect(),
            None => Vec::default(),
        }
    }
}
