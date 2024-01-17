use crate::monitor_actions::{MonitorAction, MonitorActionResult};
use chrono::{NaiveDateTime, Utc};
use shared_lib::models::full_state::FullState;

pub struct MidnightTimerCheck {}

impl MonitorAction for MidnightTimerCheck {
    fn execute(&self, full_state: &FullState) -> Option<MonitorActionResult> {
        let mut results: Vec<MonitorActionResult> = Vec::new();
        let now = Utc::now().naive_utc();

        for entries in full_state.time_entries.values() {
            for entry in entries {
                if let Some(start_time) = entry.start_time {
                    if is_past_midnight(start_time, now) {
                        results.push(MonitorActionResult::StopTimer(entry.id));
                    }
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

fn is_past_midnight(start_time: NaiveDateTime, current_time: NaiveDateTime) -> bool {
    // Check if the start time and current time are on different days
    start_time.date() < current_time.date()
}
