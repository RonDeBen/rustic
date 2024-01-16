use shared_models::{full_state::FullState, time_entry::TimeEntryVM};

use crate::monitor_actions::{MonitorAction, MonitorActionResult};

pub struct LongTimerCheck {}

impl MonitorAction for LongTimerCheck {
    fn execute(&self, full_state: &FullState) -> Option<MonitorActionResult> {
        let cutoff_millis = 10 * 60 * 60 * 1000; // 10 hours in milliseconds

        let mut results: Vec<MonitorActionResult> = Vec::new();
        for entries in full_state.time_entries.values() {
            for entry in entries {
                if has_been_running_too_long(entry, cutoff_millis) {
                    results.push(MonitorActionResult::StopTimer(entry.id));
                }
            }
        }

        if !results.is_empty() {
            Some(MonitorActionResult::Multiple(results))
        } else {
            None
        }
    }
}

fn has_been_running_too_long(entry: &TimeEntryVM, cutoff_millis: i64) -> bool {
    entry.real_total_time() > cutoff_millis
}
