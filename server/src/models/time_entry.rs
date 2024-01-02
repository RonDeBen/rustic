use chrono::{Utc, Weekday, Datelike};
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct TimeEntry {
    pub id: i32,
    pub start_time: i64,
    pub total_time: f64,
    pub note: String,
    pub day: Day,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize)]
#[repr(i8)]
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
