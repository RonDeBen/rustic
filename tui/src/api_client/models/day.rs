use chrono::{Datelike, Utc, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Eq, Hash, PartialEq, Debug)]
#[serde(from = "i16")]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl From<i16> for Day {
    fn from(day: i16) -> Self {
        match day {
            0 => Day::Monday,
            1 => Day::Tuesday,
            2 => Day::Wednesday,
            3 => Day::Thursday,
            4 => Day::Friday,
            _ => Day::default(),
        }
    }
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

impl Default for Day{
    fn default() -> Self {
        Day::Friday
    }
}
