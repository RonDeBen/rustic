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
}

#[derive(Serialize, Debug)]
pub struct TimeEntryVM {
    pub id: i32,
    pub start_time: Option<NaiveDateTime>,
    pub total_time: i64, // milliseconds
    pub note: String,
    pub day: Day,
    pub is_active: bool,
}

impl From<TimeEntryRaw> for TimeEntryVM {
    fn from(value: TimeEntryRaw) -> Self {
        Self {
            id: value.id,
            total_time: value.total_time,
            note: value.note.to_owned(),
            day: value.day,
            is_active: value.start_time.is_some(),
            start_time: value.start_time,
        }
    }
}

impl From<&TimeEntryRaw> for TimeEntryVM {
    fn from(value: &TimeEntryRaw) -> Self {
        Self {
            id: value.id,
            total_time: value.total_time,
            note: value.note.to_owned(),
            day: value.day,
            is_active: value.start_time.is_some(),
            start_time: value.start_time,
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
    pub fn get_current_day() -> Option<Self> {
        let today = Utc::now().date_naive().weekday();
        match today {
            Weekday::Mon => Some(Day::Monday),
            Weekday::Tue => Some(Day::Tuesday),
            Weekday::Wed => Some(Day::Wednesday),
            Weekday::Thu => Some(Day::Thursday),
            Weekday::Fri => Some(Day::Friday),
            _ => None,
        }
    }
}

impl From<Day> for i16 {
    fn from(value: Day) -> i16 {
        value as i16
    }
}
