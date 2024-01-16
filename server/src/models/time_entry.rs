use chrono::NaiveDateTime;
use shared_lib::models::{charge_code::ChargeCodeVM, day::Day, time_entry::TimeEntryVM};

#[derive(sqlx::FromRow, Debug)]
pub struct TimeEntryRaw {
    pub id: i32,
    pub start_time: Option<NaiveDateTime>,
    pub total_time: i64, // milliseconds
    pub note: String,
    // Fields for charge code
    pub charge_code_id: Option<i32>,
    pub day: Day,
    pub alias: Option<String>,
}

impl From<TimeEntryRaw> for TimeEntryVM {
    fn from(value: TimeEntryRaw) -> Self {
        let charge_code = match (value.charge_code_id, value.alias) {
            (Some(id), Some(alias)) => Some(ChargeCodeVM { id, alias }),
            _ => None,
        };

        Self {
            id: value.id,
            total_time: value.total_time,
            note: value.note.to_owned(),
            day: value.day,
            is_active: value.start_time.is_some(),
            start_time: value.start_time,
            charge_code,
        }
    }
}

impl From<&TimeEntryRaw> for TimeEntryVM {
    fn from(value: &TimeEntryRaw) -> Self {
        let charge_code = match (value.charge_code_id, value.alias.to_owned()) {
            (Some(id), Some(alias)) => Some(ChargeCodeVM { id, alias }),
            _ => None,
        };
        Self {
            id: value.id,
            total_time: value.total_time,
            note: value.note.to_owned(),
            day: value.day,
            is_active: value.start_time.is_some(),
            start_time: value.start_time,
            charge_code,
        }
    }
}

