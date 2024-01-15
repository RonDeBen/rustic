use chrono::{NaiveDateTime, Utc};

use crate::models::time_entry::TimeEntryRaw;

pub fn get_elapsed_time(entry: &TimeEntryRaw) -> i64 {
    match entry.start_time {
        Some(start_time) => {
            let end_time: NaiveDateTime = Utc::now().naive_utc();
            (end_time - start_time).num_milliseconds()
        }
        None => 0, // timer was "played" before it was "paused"
    }
}
