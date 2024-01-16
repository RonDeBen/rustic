use shared_lib::models::{day::Day, full_state::FullState, time_entry::TimeEntryVM};

use crate::monitor_actions::{MonitorAction, MonitorActionResult};

pub struct EodCheck {}

impl MonitorAction for EodCheck {
    fn execute(&self, full_state: &FullState) -> Option<MonitorActionResult> {
        let current_day = Day::get_current_day();
        let todays_entries = full_state.get_vms_for_day(current_day)?;
        let total_minutes = sum_to_nearest_quarter_hour(todays_entries.as_slice());

        // 7.5 hours worked
        match total_minutes >= 450 {
            true => Some(MonitorActionResult::SendMessage(
                "You are close to 8 hours worked today!!".to_string(),
            )),
            false => None,
        }
    }
}

fn sum_to_nearest_quarter_hour(entries: &[TimeEntryVM]) -> u16 {
    let total_time_millis: i64 = entries.iter().map(|entry| entry.real_total_time()).sum();
    let total_minutes = total_time_millis / 1000 / 60; // Convert milliseconds to minutes
    ((total_minutes as f64 / 15.0).round() * 15.0) as u16 // Round to nearest quarter hour
}
