use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{day::Day, charge_code::ChargeCodeVM};

#[derive(Serialize, Debug)]
pub struct TimeEntryVM {
    pub id: i32,
    pub start_time: Option<NaiveDateTime>,
    pub total_time: i64, // milliseconds
    pub note: String,
    pub day: Day,
    pub is_active: bool,
    pub charge_code: Option<ChargeCodeVM>,
}
