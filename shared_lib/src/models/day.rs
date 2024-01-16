use chrono::{Datelike, Utc, Weekday};
use serde_repr::{Deserialize_repr, Serialize_repr};

// Serialize_repr makes this serialize as the varient number, instead of a string for the day
// ditto for Deserialize_repr, but in reverse
#[derive(
    Default, Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize_repr, sqlx::Type, Deserialize_repr,
)]
#[repr(i16)]
pub enum Day {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    #[default]
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

impl From<i16> for Day {
    fn from(value: i16) -> Self {
        match value {
            0 => Day::Monday,
            1 => Day::Tuesday,
            2 => Day::Wednesday,
            3 => Day::Thursday,
            4 => Day::Friday,
            _ => Day::Friday,
        }
    }
}
