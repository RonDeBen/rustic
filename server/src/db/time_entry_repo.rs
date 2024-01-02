use crate::models::time_entry::{Day, TimeEntry};
use chrono::NaiveDateTime;
use sqlx::PgPool;

pub async fn fetch_time_entries_for_day(
    pool: &PgPool,
    day: i8,
) -> Result<Vec<TimeEntry>, sqlx::Error> {
    sqlx::query_as::<_, TimeEntry>(
        "SELECT id, start_time, total_time, note, day FROM time_tracking.time_entries WHERE day = $1"
    )
    .bind(day)
    .fetch_all(pool)
    .await
}

pub async fn fetch_time_entry_by_id(pool: &PgPool, id: i32) -> Result<TimeEntry, sqlx::Error> {
    sqlx::query_as::<_, TimeEntry>(
        "SELECT id, start_time, total_time, note, day FROM time_tracking.time_entries WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn create_time_entry(pool: &PgPool, day: Day) -> Result<TimeEntry, sqlx::Error> {
    let tx = pool.begin().await?;
    let time_entry = sqlx::query_as::<_, TimeEntry>(
        "INSERT INTO time_tracking.time_entries (start_time, total_time, note, day)
         VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(0.0)
    .bind(0.0)
    .bind("")
    .bind(day as i32)
    .fetch_one(pool)
    .await?;

    tx.commit().await?;
    Ok(time_entry)
}

pub async fn update_time_entry_note(
    pool: &PgPool,
    id: i32,
    new_note: String,
) -> Result<TimeEntry, sqlx::Error> {
    let time_entry = sqlx::query_as::<_, TimeEntry>(
        "UPDATE time_tracking.time_entries SET note = $1 WHERE id = $2 RETURNING *",
    )
    .bind(new_note)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(time_entry)
}

pub async fn play_time_entry(
    pool: &PgPool,
    id: i32,
    start_time: NaiveDateTime,
) -> Result<TimeEntry, sqlx::Error> {
    let time_entry = sqlx::query_as::<_, TimeEntry>(
        "UPDATE time_tracking.time_entries SET start_time = $1 WHERE id = $2 RETURNING *",
    )
    .bind(start_time)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(time_entry)
}

pub async fn pause_time_entry(
    pool: &PgPool,
    id: i32,
    total_time: i64,
) -> Result<TimeEntry, sqlx::Error> {
    let time_entry = sqlx::query_as::<_, TimeEntry>(
        "UPDATE time_tracking.time_entries SET total_time = $1 WHERE id = $2 RETURNING *",
    )
    .bind(total_time)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(time_entry)
}

pub async fn delete_time_entry(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM time_tracking.time_entries WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
