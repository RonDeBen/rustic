use std::collections::HashMap;

use crate::db::time_entry_repo::update_time_entry_note;
use crate::utils::error::Result;
use crate::{
    db::time_entry_repo::*,
    models::time_entry::{Day, TimeEntry},
    utils::error::AppError,
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdPath {
    id: i32,
}

#[derive(Deserialize)]
pub struct DayQuery {
    day: i16,
}

#[derive(Deserialize)]
pub struct NoteQuery {
    note: String,
}

pub async fn get_time_entries_by_day(
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<HashMap<Day, Vec<TimeEntry>>>> {
    let entries = fetch_all_time_entries(&*pool).await?;
    let organized_entries = organize_time_entries_by_day(entries);

    Ok(Json(organized_entries))
}

pub async fn get_time_entries_request(
    Query(params): Query<DayQuery>,
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<Vec<TimeEntry>>> {
    let records = fetch_time_entries_for_day(&*pool, params.day).await?;

    Ok(Json(records))
}

pub async fn create_time_entry_request(pool: Extension<sqlx::PgPool>) -> Result<Json<TimeEntry>> {
    let day = Day::get_current_day().ok_or(AppError::WeekendError)?;
    let entry = create_time_entry(&*pool, day).await?;

    Ok(Json(entry))
}

pub async fn update_time_entry_note_request(
    Path(params): Path<IdPath>,
    Query(query): Query<NoteQuery>,
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<TimeEntry>> {
    let entry = update_time_entry_note(&*pool, params.id, query.note).await?;

    Ok(Json(entry))
}

pub async fn play_time_entry_request(
    Path(params): Path<IdPath>,
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<TimeEntry>> {
    let start_time: NaiveDateTime = Utc::now().naive_utc();
    let entry = play_time_entry(&*pool, params.id, start_time).await?;

    Ok(Json(entry))
}

pub async fn pause_time_entry_request(
    Path(params): Path<IdPath>,
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<TimeEntry>> {
    let entry = fetch_time_entry_by_id(&*pool, params.id).await?;
    let elapsed_time = match entry.start_time {
        Some(start_time) => {
            let end_time: NaiveDateTime = Utc::now().naive_utc();
            (end_time - start_time).num_milliseconds()
        }
        None => 0, // timer was "played" before it was "paused"
    };
    let entry = pause_time_entry(&*pool, params.id, elapsed_time).await?;

    Ok(Json(entry))
}

pub async fn delete_time_entry_request(
    Path(params): Path<IdPath>,
    pool: Extension<sqlx::PgPool>,
) -> Result<()> {
    delete_time_entry(&*pool, params.id).await?;
    Ok(())
}
