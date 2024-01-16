use crate::{
    db::time_entry_repo::{
        fetch_all_running_timers, fetch_time_entries_for_day, pause_time_entry,
        play_time_entry_and_return_day,
    },
    models::{time_entry::TimeEntryRaw, DayTimeEntries},
    utils::time::get_elapsed_time,
};
use chrono::{NaiveDateTime, Utc};
use shared_lib::models::day::Day;
use sqlx::PgPool;

pub async fn switch_to_timer(pool: &PgPool, id: i32) -> Result<DayTimeEntries, sqlx::Error> {
    // pause all running timers
    let running_timers = fetch_all_running_timers(pool).await?;
    for timer in running_timers {
        pause_timer(pool, &timer).await?;
    }

    // start new timer
    let start_time: NaiveDateTime = Utc::now().naive_utc();
    let day = play_time_entry_and_return_day(pool, id, start_time).await?;

    // return current state of timers for this day
    let entries = fetch_time_entries_for_day(pool, day).await?;

    let day = match entries.first() {
        Some(entry) => entry.day,
        None => Day::Friday,
    };

    Ok(DayTimeEntries::new(day, entries.as_slice()))
}

async fn pause_timer(pool: &PgPool, entry: &TimeEntryRaw) -> Result<(), sqlx::Error> {
    let elapsed_time = get_elapsed_time(entry);
    pause_time_entry(pool, entry.id, elapsed_time).await?;

    Ok(())
}
