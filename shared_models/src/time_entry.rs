use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use crate::{charge_code::ChargeCodeVM, day::Day};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeEntryVM {
    pub id: i32,
    pub start_time: Option<NaiveDateTime>,
    pub total_time: i64, // milliseconds
    pub note: String,
    pub day: Day,
    pub is_active: bool,
    pub charge_code: Option<ChargeCodeVM>,
}

impl Eq for TimeEntryVM {}

impl Hash for TimeEntryVM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl TimeEntryVM {
    pub fn real_total_time(&self) -> i64 {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let elapsed_since_start = self
            .start_time
            .map(|start| now.signed_duration_since(start).num_milliseconds())
            .unwrap_or(0);

        self.total_time + elapsed_since_start
    }
}
