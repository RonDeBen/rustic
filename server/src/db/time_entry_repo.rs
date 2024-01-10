use crate::models::time_entry::{Day, TimeEntryRaw, TimeEntryVM};
use chrono::NaiveDateTime;
use sqlx::{Executor, Postgres};
use std::collections::HashMap;

pub async fn fetch_all_time_entries<'e, E>(exec: E) -> Result<Vec<TimeEntryRaw>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, TimeEntryRaw>(
        "SELECT * FROM time_tracking.time_entries ORDER BY day, start_time",
    )
    .fetch_all(exec)
    .await
}

pub async fn fetch_all_running_timers<'e, E>(exec: E) -> Result<Vec<TimeEntryRaw>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, TimeEntryRaw>(
        "SELECT * FROM time_tracking.time_entries WHERE start_time IS NOT NULL",
    )
    .fetch_all(exec)
    .await
}

pub fn organize_time_entries_by_day(entries: Vec<TimeEntryRaw>) -> HashMap<Day, Vec<TimeEntryVM>> {
    let mut map: HashMap<Day, Vec<TimeEntryVM>> = HashMap::new();

    for entry in entries {
        map.entry(entry.day).or_default().push(entry.into());
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
        "SELECT id, start_time, total_time, note, day FROM time_tracking.time_entries WHERE day = $1"
    )
    .bind(day)
    .fetch_all(exec)
    .await
}

pub async fn fetch_time_entry_by_id<'e, E>(exec: E, id: i32) -> Result<TimeEntryRaw, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, TimeEntryRaw>(
        "SELECT id, start_time, total_time, note, day FROM time_tracking.time_entries WHERE id = $1"
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
        "INSERT INTO time_tracking.time_entries (start_time, total_time, note, day)
         VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(None::<NaiveDateTime>)
    .bind(0.0)
    .bind("")
    .bind(day as i16)
    .fetch_one(exec)
    .await?;

    Ok(time_entry)
}

pub async fn update_time_entry_note<'e, E>(
    exec: E,
    id: i32,
    new_note: String,
) -> Result<TimeEntryRaw, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let time_entry = sqlx::query_as::<_, TimeEntryRaw>(
        "UPDATE time_tracking.time_entries SET note = $1 WHERE id = $2 RETURNING *",
    )
    .bind(new_note)
    .bind(id)
    .fetch_one(exec)
    .await?;

    Ok(time_entry)
}

pub async fn play_time_entry<'e, E>(
    exec: E,
    id: i32,
    start_time: NaiveDateTime,
) -> Result<TimeEntryRaw, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let time_entry = sqlx::query_as::<_, TimeEntryRaw>(
        "UPDATE time_tracking.time_entries SET start_time = $1 WHERE id = $2 RETURNING *",
    )
    .bind(start_time)
    .bind(id)
    .fetch_one(exec)
    .await?;

    Ok(time_entry)
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

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::db::time_entry_repo::*;
    use crate::utils::connections::get_connection;

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
        let updated_entry = update_time_entry_note(&mut *tx, entry.id, "new note".to_string())
            .await
            .unwrap();

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
        let _ = play_time_entry(&mut *tx, entry.id, start_time)
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
        let _ = play_time_entry(&mut *tx, entry.id, start_time)
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
        let _ = play_time_entry(&mut *tx, entry.id, start_time)
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
        let started_timer = play_time_entry(&mut *tx, entry.id, start_time)
            .await
            .unwrap();

        let running_timers = fetch_all_running_timers(&mut *tx).await.unwrap();

        assert_eq!(running_timers.len(), 1);
        assert_eq!(started_timer.id, running_timers.first().unwrap().id);

        tx.rollback().await.unwrap()
    }
}

