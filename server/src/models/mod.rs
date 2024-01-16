// use shared_models::full_state::DayTimeEntries as SharedDayTimeEntries;
use self::time_entry::TimeEntryRaw;
use shared_models::time_entry::TimeEntryVM;
use shared_models::day::Day;

pub mod time_entry;

#[derive(serde::Serialize, Debug)]
pub struct DayTimeEntries {
    pub day: Day,
    pub entries: Vec<TimeEntryVM>,
}

impl DayTimeEntries {
    pub fn new(day: Day, entries: &[TimeEntryRaw]) -> Self {
        let mut vms: Vec<TimeEntryVM> = entries.iter().map(|x| x.into()).collect();
        vms.sort_by(|a, b| a.id.cmp(&b.id));
        Self { day, entries: vms }
    }
}
