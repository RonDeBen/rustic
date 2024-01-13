use chrono::{Datelike, NaiveDateTime, Utc, Weekday};
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(sqlx::FromRow, Debug)]
pub struct TimeEntryRaw {
    pub id: i32,
    pub start_time: Option<NaiveDateTime>,
    pub total_time: i64, // milliseconds
    pub note: String,
    pub day: Day,
    // Fields for charge code
    pub charge_code_id: Option<i32>,
    pub alias: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ChargeCodeVM {
    pub id: i32,
    pub alias: String,
}

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

//Serialize_repr makes this serialize as the varient number, instead of a string for the day
#[derive(Debug, Clone, Copy, sqlx::Type, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(i16)]
pub enum Day {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
}

impl Day {
    pub fn get_current_day() -> Self {
        let today = Utc::now().date_naive().weekday();
        match today {
            Weekday::Mon => Day::Monday,
            Weekday::Tue => Day::Tuesday,
            Weekday::Wed => Day::Wednesday,
            Weekday::Thu => Day::Thursday,
            Weekday::Fri => Day::Friday,
            _ => Day::Friday,
        }
    }
}

impl From<Day> for i16 {
    fn from(value: Day) -> i16 {
        value as i16
    }
}
