use sqlx::PgPool;

use crate::models::time_entry::TimeEntry;

pub async fn fetch_time_entries_for_day(
    pool: &PgPool,
    day: i32,
) -> Result<Vec<TimeEntry>, sqlx::Error> {
    sqlx::query_as::<_, TimeEntry>(
        "SELECT id, start_time, total_time, note, day FROM time_tracking.time_entries WHERE day = $1"
    )
    .bind(day)
    .fetch_all(pool)
    .await
}
