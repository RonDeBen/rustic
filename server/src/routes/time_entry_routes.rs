use crate::{db::time_entry_repo::fetch_time_entries_for_day, models::time_entry::TimeEntry};
use axum::{extract::Query, Json};
use serde::Deserialize;
use crate::utils::error::Result;

#[derive(Deserialize)]
pub struct TimeEntryQuery {
    day: i32,
}

pub async fn get_time_entries(
    Query(params): Query<TimeEntryQuery>,
    pool: axum::extract::Extension<sqlx::PgPool>,
) -> Result<Json<Vec<TimeEntry>>> {
    let records = fetch_time_entries_for_day(&pool, params.day).await?;

    Ok(Json(records))
}

