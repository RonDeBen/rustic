use crate::models::{costpoint_entry::CostpointEntryRaw, time_entry::TimeEntryRaw};
use chrono::NaiveDateTime;
use shared_lib::models::{day::Day, time_entry::TimeEntryVM};
use sqlx::{Executor, Postgres};
use std::collections::HashMap;

pub async fn fetch_all_time_entries<'e, E>(exec: E) -> Result<Vec<TimeEntryRaw>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, TimeEntryRaw>(
        "SELECT te.id, te.start_time, te.total_time, te.note, te.day, cc.id as charge_code_id, cc.alias
         FROM time_tracking.time_entries te
         LEFT JOIN time_tracking.charge_codes cc ON te.charge_code_id = cc.id"
    )
    .fetch_all(exec)
    .await
}

pub async fn fetch_all_running_timers<'e, E>(exec: E) -> Result<Vec<TimeEntryRaw>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, TimeEntryRaw>(
        "SELECT te.id, te.start_time, te.total_time, te.note, te.day, cc.id as charge_code_id, cc.alias
         FROM time_tracking.time_entries te
         LEFT JOIN time_tracking.charge_codes cc ON te.charge_code_id = cc.id
         WHERE te.start_time IS NOT NULL",
    )
    .fetch_all(exec)
    .await
}

pub fn organize_time_entries_by_day(entries: Vec<TimeEntryRaw>) -> HashMap<Day, Vec<TimeEntryVM>> {
    let mut map: HashMap<Day, Vec<TimeEntryVM>> = HashMap::new();

    for entry in entries {
        map.entry(entry.day).or_default().push(entry.into());
    }

    for vms in map.values_mut() {
        vms.sort_by(|a, b| a.id.cmp(&b.id));
    }

    map
}

pub async fn fetch_time_entries_for_day<'e, E>(
    exec: E,
    day: i16,
) -> Result<Vec<TimeEntryRaw>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, TimeEntryRaw>(
        "SELECT te.id, te.start_time, te.total_time, te.note, te.day, cc.id as charge_code_id, cc.alias
         FROM time_tracking.time_entries te
         LEFT JOIN time_tracking.charge_codes cc ON te.charge_code_id = cc.id
         WHERE te.day = $1"
    )
    .bind(day)
    .fetch_all(exec)
    .await
}

pub async fn upsert_time_entry<'e, E>(exec: E, update: TimeEntryVM) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "INSERT INTO time_tracking.time_entries (id, start_time, total_time, note, day, charge_code_id)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (id) DO UPDATE SET
         start_time = EXCLUDED.start_time,
         total_time = EXCLUDED.total_time,
         note = EXCLUDED.note,
         day = EXCLUDED.day,
         charge_code_id = EXCLUDED.charge_code_id
         RETURNING id, start_time, total_time, note, day, charge_code_id"
    )
    .bind(update.id)
    .bind(update.start_time)
    .bind(update.total_time)
    .bind(update.note)
    .bind(update.day as i16)
    .bind(update.charge_code.as_ref().map(|x| x.id))
    .execute(exec)
    .await?;

    Ok(())
}

pub async fn fetch_time_entry_by_id<'e, E>(exec: E, id: i32) -> Result<TimeEntryRaw, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, TimeEntryRaw>(
        "SELECT te.id, te.start_time, te.total_time, te.note, te.day, cc.id as charge_code_id, cc.alias
         FROM time_tracking.time_entries te
         LEFT JOIN time_tracking.charge_codes cc ON te.charge_code_id = cc.id
         WHERE te.id = $1"
    )
    .bind(id)
    .fetch_one(exec)
    .await
}

pub async fn create_time_entry<'e, E>(exec: E, day: Day) -> Result<TimeEntryRaw, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let time_entry = sqlx::query_as::<_, TimeEntryRaw>(
        "INSERT INTO time_tracking.time_entries (start_time, total_time, note, day, created_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, start_time, total_time, note, day, null as charge_code_id, null as alias",
    )
    .bind(None::<NaiveDateTime>)
    .bind(0.0)
    .bind("")
    .bind(day as i16)
    .bind(day.into_date())
    .fetch_one(exec)
    .await?;

    Ok(time_entry)
}

pub async fn update_charge_code_for_time_entry<'e, E>(
    exec: E,
    entry_id: i32,
    charge_code_id: i32,
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    // First, update the charge code for the time entry
    sqlx::query(
        "UPDATE time_tracking.time_entries
         SET charge_code_id = $2
         WHERE id = $1",
    )
    .bind(entry_id)
    .bind(charge_code_id)
    .execute(exec)
    .await?;

    Ok(())
}

pub async fn update_time_for_time_entry<'e, E>(
    exec: E,
    entry_id: i32,
    total_time: i64,
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "UPDATE time_tracking.time_entries
         SET total_time = $2
         WHERE id = $1",
    )
    .bind(entry_id)
    .bind(total_time)
    .execute(exec)
    .await?;

    Ok(())
}

pub async fn add_time_to_entry<'e, E>(
    exec: E,
    entry_id: i32,
    add_time: i64,
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "UPDATE time_tracking.time_entries
         SET total_time = GREATEST(0, total_time + $2)
         WHERE id = $1",
    )
    .bind(entry_id)
    .bind(add_time)
    .execute(exec)
    .await?;

    Ok(())
}

pub async fn update_time_entry_note<'e, E>(
    exec: E,
    id: i32,
    new_note: String,
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE time_tracking.time_entries SET note = $1 WHERE id = $2")
        .bind(new_note)
        .bind(id)
        .execute(exec)
        .await?;

    Ok(())
}

pub async fn play_time_entry_and_return_day<'e, E>(
    exec: E,
    id: i32,
    start_time: NaiveDateTime,
) -> Result<i16, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let (day,): (i16,) = sqlx::query_as(
        "UPDATE time_tracking.time_entries SET start_time = $1 WHERE id = $2 RETURNING day",
    )
    .bind(start_time)
    .bind(id)
    .fetch_one(exec)
    .await?;

    Ok(day)
}

pub async fn pause_time_entry<'e, E>(exec: E, id: i32, elapsed_time: i64) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "UPDATE time_tracking.time_entries SET total_time = total_time + $1, start_time = NULL WHERE id = $2",
    )
    .bind(elapsed_time)
    .bind(id)
    .execute(exec)
    .await?;

    Ok(())
}

pub async fn delete_time_entry<'e, E>(exec: E, id: i32) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("DELETE FROM time_tracking.time_entries WHERE id = $1")
        .bind(id)
        .execute(exec)
        .await?;

    Ok(())
}

pub async fn delete_old_time_entries<'e, E>(exec: E) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "DELETE FROM time_tracking.time_entries
         WHERE created_at < NOW() - INTERVAL '7 days'",
    )
    .execute(exec)
    .await?;

    Ok(())
}

pub async fn fetch_costpoint_entries<'e, E>(exec: E) -> Result<Vec<CostpointEntryRaw>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let entries = sqlx::query_as::<_, CostpointEntryRaw>(
"
SELECT
    cc.code AS charge_code,
    CAST(
        SUM(
            COALESCE(
                te.total_time + CASE WHEN te.start_time IS NOT NULL THEN EXTRACT(EPOCH FROM (NOW() - te.start_time)) * 1000 ELSE 0 END,
                te.total_time
            )
        ) AS BIGINT
    ) AS total_time_milliseconds,
    TO_CHAR(te.created_at, 'MM/DD/YY') AS entry_date,
    STRING_AGG(te.note, '\n') AS notes
FROM
    time_tracking.time_entries te
LEFT JOIN
    time_tracking.charge_codes cc ON te.charge_code_id = cc.id
GROUP BY
    cc.code, TO_CHAR(te.created_at, 'MM/DD/YY');
",
    )
    .fetch_all(exec)
    .await?;

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use crate::db::time_entry_repo::*;
    use crate::utils::connections::get_connection;
    use chrono::Utc;

    #[tokio::test]
    async fn can_create_new_entry() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let start_count = fetch_time_entries_for_day(&mut *tx, Day::Monday.into())
            .await
            .unwrap()
            .len();
        let _entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let end_count = fetch_time_entries_for_day(&mut *tx, Day::Monday.into())
            .await
            .unwrap()
            .len();

        assert_eq!(start_count, 0);
        assert_eq!(end_count, 1);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn can_fetch_entries_by_day() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let _entry_monday_1 = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let _entry_monday_2 = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let _entry_tuesday_1 = create_time_entry(&mut *tx, Day::Tuesday).await.unwrap();
        let monday_count = fetch_time_entries_for_day(&mut *tx, Day::Monday.into())
            .await
            .unwrap()
            .len();

        assert_eq!(monday_count, 2);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn can_fetch_entry_by_id() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let found_entry = fetch_time_entry_by_id(&mut *tx, entry.id).await.unwrap();

        assert_eq!(entry.id, found_entry.id);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn can_update_notes() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        update_time_entry_note(&mut *tx, entry.id, "new note".to_string())
            .await
            .unwrap();
        let updated_entry = fetch_time_entry_by_id(&mut *tx, entry.id).await.unwrap();

        assert_ne!(entry.note, updated_entry.note);
        assert_eq!(updated_entry.note, "new note".to_string());

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn pausing_updates_elapsed_time() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let start_time: NaiveDateTime = Utc::now().naive_utc();
        play_time_entry_and_return_day(&mut *tx, entry.id, start_time)
            .await
            .unwrap();
        let ten_min_millis = 600000;
        pause_time_entry(&mut *tx, entry.id, ten_min_millis)
            .await
            .unwrap();
        let paused_entry = fetch_time_entry_by_id(&mut *tx, entry.id).await.unwrap();

        assert_eq!(paused_entry.total_time, ten_min_millis);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn pausing_adds_to_toal_time() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let start_time: NaiveDateTime = Utc::now().naive_utc();
        play_time_entry_and_return_day(&mut *tx, entry.id, start_time)
            .await
            .unwrap();
        let ten_min_millis = 600000;
        pause_time_entry(&mut *tx, entry.id, ten_min_millis)
            .await
            .unwrap();
        let paused_entry = fetch_time_entry_by_id(&mut *tx, entry.id).await.unwrap();

        // gets first 10 millis
        assert_eq!(paused_entry.total_time, ten_min_millis);

        let start_time: NaiveDateTime = Utc::now().naive_utc();
        play_time_entry_and_return_day(&mut *tx, entry.id, start_time)
            .await
            .unwrap();
        let ten_min_millis = 600000;
        pause_time_entry(&mut *tx, entry.id, ten_min_millis)
            .await
            .unwrap();
        let paused_entry = fetch_time_entry_by_id(&mut *tx, entry.id).await.unwrap();

        // should be 20 minutes later now
        assert_eq!(paused_entry.total_time, 2 * ten_min_millis);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn can_delete_time_entries() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let start_count = fetch_time_entries_for_day(&mut *tx, Day::Monday.into())
            .await
            .unwrap()
            .len();
        let entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let end_count = fetch_time_entries_for_day(&mut *tx, Day::Monday.into())
            .await
            .unwrap()
            .len();

        assert_eq!(start_count, 0);
        assert_eq!(end_count, 1);

        delete_time_entry(&mut *tx, entry.id).await.unwrap();

        let after_delete_count = fetch_time_entries_for_day(&mut *tx, Day::Monday.into())
            .await
            .unwrap()
            .len();

        assert_eq!(after_delete_count, 0);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn can_get_running_timers() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let _entry2 = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
        let _entry3 = create_time_entry(&mut *tx, Day::Monday).await.unwrap();

        let start_time: NaiveDateTime = Utc::now().naive_utc();
        play_time_entry_and_return_day(&mut *tx, entry.id, start_time)
            .await
            .unwrap();

        let running_timers = fetch_all_running_timers(&mut *tx).await.unwrap();
        let started_timer = fetch_time_entry_by_id(&mut *tx, entry.id).await.unwrap();

        assert_eq!(running_timers.len(), 1);
        assert_eq!(started_timer.id, running_timers.first().unwrap().id);

        tx.rollback().await.unwrap()
    }
}
