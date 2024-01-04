use std::collections::HashMap;
use self::{time_entry::{Day, TimeEntry}, charge_code::ChargeCode};

pub mod charge_code;
pub mod time_entry;

#[derive(serde::Serialize)]
pub struct FullState{
    pub time_entries: HashMap<Day, Vec<TimeEntry>>,
    pub charge_codes: Vec<ChargeCode>,
}
