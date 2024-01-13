use crate::{
    db::time_entry_repo::{fetch_all_running_timers, fetch_time_entries_for_day, pause_time_entry},
    models::{time_entry::TimeEntryRaw, DayTimeEntries},
};
use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;

pub async fn switch_to_timer(pool: &PgPool, id: i32) -> Result<DayTimeEntries, sqlx::Error> {
    // pause all running timers
    let running_timers = fetch_all_running_timers(pool).await?;
    for timer in running_timers {
        pause_timer(pool, &timer).await?;
    }

    // start new timer
    let start_time: NaiveDateTime = Utc::now().naive_utc();
    let entries = play_and_fetch_day_entries(pool, id, start_time).await?;

    let day = match entries.first() {
        Some(entry) => entry.day,
        None => crate::models::time_entry::Day::Friday,
    };

    Ok(DayTimeEntries::new(day, entries.as_slice()))
}

pub async fn pause_timer(pool: &PgPool, entry: &TimeEntryRaw) -> Result<(), sqlx::Error> {
    let elapsed_time = get_elapsed_time(entry);
    pause_time_entry(pool, entry.id, elapsed_time).await?;

    Ok(())
}

pub async fn pause_timer_and_get_entries(
    pool: &PgPool,
    entry: &TimeEntryRaw,
) -> Result<DayTimeEntries, sqlx::Error> {
    let elapsed_time = get_elapsed_time(entry);
    let entries = pause_and_fetch_day_entries(pool, entry.id, elapsed_time).await?;

    Ok(DayTimeEntries::new(entry.day, entries.as_slice()))
}

fn get_elapsed_time(entry: &TimeEntryRaw) -> i64 {
    match entry.start_time {
        Some(start_time) => {
            let end_time: NaiveDateTime = Utc::now().naive_utc();
            (end_time - start_time).num_milliseconds()
        }
        None => 0, // timer was "played" before it was "paused"
    }
}

async fn play_and_fetch_day_entries(
    pool: &PgPool,
    id: i32,
    start_time: NaiveDateTime,
) -> Result<Vec<TimeEntryRaw>, sqlx::Error> {
    let (day,): (i16,) = sqlx::query_as(
        "UPDATE time_tracking.time_entries SET start_time = $1 WHERE id = $2 RETURNING day",
    )
    .bind(start_time)
    .bind(id)
    .fetch_one(pool)
    .await?;

    let entries = fetch_time_entries_for_day(pool, day).await?;

    Ok(entries)
}

pub async fn pause_and_fetch_day_entries(
    pool: &PgPool,
    id: i32,
    elapsed_time: i64,
) -> Result<Vec<TimeEntryRaw>, sqlx::Error> {
    let (day,): (i16,) = sqlx::query_as(
        "UPDATE time_tracking.time_entries SET total_time = total_time + $1, start_time = NULL WHERE id = $2 RETURNING day",
    )
    .bind(elapsed_time)
    .bind(id)
    .fetch_one(pool)
    .await?;

    let entries = fetch_time_entries_for_day(pool, day).await?;

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use crate::db::time_entry_repo::*;
    use crate::models::time_entry::Day;
    use crate::services::time_entry_service::*;
    use crate::utils::connections::get_connection;
    use chrono::NaiveDateTime;
    use chrono::Utc;
    use std::time::Duration;

    #[tokio::test]
    async fn pausing_correctly_calculates_elapsed_time() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let start_time: NaiveDateTime = Utc::now().naive_utc();
        let played_entries = play_and_fetch_day_entries(&pool, entry.id, start_time)
            .await
            .unwrap();
        let played_entry = played_entries.iter().find(|x| x.id == entry.id).unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;
        let paused_entries = pause_timer_and_get_entries(&pool, played_entry)
            .await
            .unwrap();
        let paused_entry = paused_entries
            .entries
            .iter()
            .find(|x| x.id == entry.id)
            .unwrap();

        // is around 1 second
        assert!(paused_entry.total_time >= 995);
        assert!(paused_entry.total_time < 1005);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn starting_a_timer_pauses_other_timers() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry1 = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let entry2 = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let start_time: NaiveDateTime = Utc::now().naive_utc();
        let _ = play_time_entry(&pool, entry1.id, start_time).await.unwrap();

        let swapped_to_entry2_entries = switch_to_timer(&pool, entry2.id).await.unwrap();
        let swapped_to_entry2 = swapped_to_entry2_entries
            .entries
            .iter()
            .find(|x| x.id == entry2.id)
            .unwrap();

        let running_timers = fetch_all_running_timers(&mut *tx).await.unwrap();

        assert_eq!(running_timers.len(), 1);
        assert_eq!(running_timers.first().unwrap().id, swapped_to_entry2.id);

        tx.rollback().await.unwrap()
    }
}
