use crate::models::time_entry::{Day, TimeEntry};
use chrono::NaiveDateTime;
use sqlx::{Acquire, Postgres};
use std::collections::HashMap;

pub async fn fetch_all_time_entries<'a, A>(conn: A) -> Result<Vec<TimeEntry>, sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;
    sqlx::query_as::<_, TimeEntry>(
        "SELECT * FROM time_tracking.time_entries ORDER BY day, start_time",
    )
    .fetch_all(&mut *pool)
    .await
}

pub async fn fetch_all_running_timers<'a, A>(conn: A) -> Result<Vec<TimeEntry>, sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;
    sqlx::query_as::<_, TimeEntry>(
        "SELECT * FROM time_tracking.time_entries WHERE start_time IS NOT NULL",
    )
    .fetch_all(&mut *pool)
    .await
}

pub fn organize_time_entries_by_day(entries: Vec<TimeEntry>) -> HashMap<Day, Vec<TimeEntry>> {
    let mut map: HashMap<Day, Vec<TimeEntry>> = HashMap::new();

    for entry in entries {
        map.entry(entry.day).or_default().push(entry);
    }

    map
}

pub async fn fetch_time_entries_for_day<'a, A>(
    conn: A,
    day: i16,
) -> Result<Vec<TimeEntry>, sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;

    sqlx::query_as::<_, TimeEntry>(
        "SELECT id, start_time, total_time, note, day FROM time_tracking.time_entries WHERE day = $1"
    )
    .bind(day)
    .fetch_all(&mut *pool)
    .await
}

pub async fn fetch_time_entry_by_id<'a, A>(conn: A, id: i32) -> Result<TimeEntry, sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;
    sqlx::query_as::<_, TimeEntry>(
        "SELECT id, start_time, total_time, note, day FROM time_tracking.time_entries WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&mut *pool)
    .await
}

pub async fn create_time_entry<'a, A>(conn: A, day: Day) -> Result<TimeEntry, sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;
    let time_entry = sqlx::query_as::<_, TimeEntry>(
        "INSERT INTO time_tracking.time_entries (start_time, total_time, note, day)
         VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(None::<NaiveDateTime>)
    .bind(0.0)
    .bind("")
    .bind(day as i16)
    .fetch_one(&mut *pool)
    .await?;

    Ok(time_entry)
}

pub async fn update_time_entry_note<'a, A>(
    conn: A,
    id: i32,
    new_note: String,
) -> Result<TimeEntry, sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;
    let time_entry = sqlx::query_as::<_, TimeEntry>(
        "UPDATE time_tracking.time_entries SET note = $1 WHERE id = $2 RETURNING *",
    )
    .bind(new_note)
    .bind(id)
    .fetch_one(&mut *pool)
    .await?;

    Ok(time_entry)
}

pub async fn play_time_entry<'a, A>(
    conn: A,
    id: i32,
    start_time: NaiveDateTime,
) -> Result<TimeEntry, sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;
    let time_entry = sqlx::query_as::<_, TimeEntry>(
        "UPDATE time_tracking.time_entries SET start_time = $1 WHERE id = $2 RETURNING *",
    )
    .bind(start_time)
    .bind(id)
    .fetch_one(&mut *pool)
    .await?;

    Ok(time_entry)
}

pub async fn pause_time_entry<'a, A>(
    conn: A,
    id: i32,
    elapsed_time: i64,
) -> Result<TimeEntry, sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;
    let time_entry = sqlx::query_as::<_, TimeEntry>(
        "UPDATE time_tracking.time_entries SET total_time = total_time + $1, start_time = NULL WHERE id = $2 RETURNING *",
    )
    .bind(elapsed_time)
    .bind(id)
    .fetch_one(&mut *pool)
    .await?;

    Ok(time_entry)
}

pub async fn delete_time_entry<'a, A>(conn: A, id: i32) -> Result<(), sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut pool = conn.acquire().await?;
    sqlx::query("DELETE FROM time_tracking.time_entries WHERE id = $1")
        .bind(id)
        .execute(&mut *pool)
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

        let start_count = fetch_time_entries_for_day(&mut tx, Day::Monday.into())
            .await
            .unwrap()
            .len();
        let _entry = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let end_count = fetch_time_entries_for_day(&mut tx, Day::Monday.into())
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

        let _entry_monday_1 = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let _entry_monday_2 = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let _entry_tuesday_1 = create_time_entry(&mut tx, Day::Tuesday).await.unwrap();
        let monday_count = fetch_time_entries_for_day(&mut tx, Day::Monday.into())
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

        let entry = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let found_entry = fetch_time_entry_by_id(&mut tx, entry.id).await.unwrap();

        assert_eq!(entry.id, found_entry.id);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn can_update_notes() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let updated_entry = update_time_entry_note(&mut tx, entry.id, "new note".to_string())
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

        let entry = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let start_time: NaiveDateTime = Utc::now().naive_utc();
        let _ = play_time_entry(&mut tx, entry.id, start_time)
            .await
            .unwrap();
        let ten_min_millis = 600000;
        let paused_entry = pause_time_entry(&mut tx, entry.id, ten_min_millis)
            .await
            .unwrap();

        assert_eq!(paused_entry.total_time, ten_min_millis);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn pausing_adds_to_toal_time() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let entry = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let start_time: NaiveDateTime = Utc::now().naive_utc();
        let _ = play_time_entry(&mut tx, entry.id, start_time)
            .await
            .unwrap();
        let ten_min_millis = 600000;
        let paused_entry = pause_time_entry(&mut tx, entry.id, ten_min_millis)
            .await
            .unwrap();

        // gets first 10 millis
        assert_eq!(paused_entry.total_time, ten_min_millis);

        let start_time: NaiveDateTime = Utc::now().naive_utc();
        let _ = play_time_entry(&mut tx, entry.id, start_time)
            .await
            .unwrap();
        let ten_min_millis = 600000;
        let paused_entry = pause_time_entry(&mut tx, entry.id, ten_min_millis)
            .await
            .unwrap();

        // should be 20 minutes later now
        assert_eq!(paused_entry.total_time, 2 * ten_min_millis);

        tx.rollback().await.unwrap()
    }

    #[tokio::test]
    async fn can_delete_time_entries() {
        let pool = get_connection().await;
        let mut tx = pool.begin().await.unwrap();

        let start_count = fetch_time_entries_for_day(&mut tx, Day::Monday.into())
            .await
            .unwrap()
            .len();
        let entry = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let end_count = fetch_time_entries_for_day(&mut tx, Day::Monday.into())
            .await
            .unwrap()
            .len();

        assert_eq!(start_count, 0);
        assert_eq!(end_count, 1);

        delete_time_entry(&mut tx, entry.id).await.unwrap();

        let after_delete_count = fetch_time_entries_for_day(&mut tx, Day::Monday.into())
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

        let entry = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let _entry2 = create_time_entry(&mut tx, Day::Monday).await.unwrap();
        let _entry3 = create_time_entry(&mut tx, Day::Monday).await.unwrap();

        let start_time: NaiveDateTime = Utc::now().naive_utc();
        let started_timer = play_time_entry(&mut tx, entry.id, start_time)
            .await
            .unwrap();

        let running_timers = fetch_all_running_timers(&mut tx).await.unwrap();

        assert_eq!(running_timers.len(), 1);
        assert_eq!(started_timer.id, running_timers.first().unwrap().id);

        tx.rollback().await.unwrap()
    }

    // TODO: make a test for stopping all other timers
    // and update the code to make that pass
}
