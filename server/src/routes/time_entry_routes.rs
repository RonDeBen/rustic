use crate::db::charge_code_repo::fetch_charge_codes;
use crate::db::time_entry_repo::update_time_entry_note;
use crate::models::FullState;
use crate::services::time_entry_service::{pause_timer, switch_to_timer};
use crate::utils::error::Result;
use crate::{
    db::time_entry_repo::*,
    models::time_entry::{Day, TimeEntryVM},
    utils::error::AppError,
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use serde::Deserialize;
use std::collections::HashMap;

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

pub async fn get_everything_request(pool: Extension<sqlx::PgPool>) -> Result<Json<FullState>> {
    let entries = fetch_all_time_entries(&*pool).await?;
    let time_entries = organize_time_entries_by_day(entries);
    let charge_codes = fetch_charge_codes(&pool).await?;

    let full_state = FullState {
        time_entries,
        charge_codes,
    };

    Ok(Json(full_state))
}

pub async fn get_time_entries_by_day_request(
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<HashMap<Day, Vec<TimeEntryVM>>>> {
    let entries = fetch_all_time_entries(&*pool).await?;
    let organized_entries = organize_time_entries_by_day(entries);

    Ok(Json(organized_entries))
}

pub async fn get_time_entries_request(
    Query(params): Query<DayQuery>,
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<Vec<TimeEntryVM>>> {
    let records = fetch_time_entries_for_day(&*pool, params.day).await?;
    let vms: Vec<TimeEntryVM> = records.iter().map(|x| x.into()).collect();

    Ok(Json(vms))
}

pub async fn create_time_entry_request(pool: Extension<sqlx::PgPool>) -> Result<Json<TimeEntryVM>> {
    let day = Day::get_current_day().ok_or(AppError::WeekendError)?;
    let entry = create_time_entry(&*pool, day).await?;

    Ok(Json(entry.into()))
}

pub async fn update_time_entry_note_request(
    Path(params): Path<IdPath>,
    Query(query): Query<NoteQuery>,
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<TimeEntryVM>> {
    let entry = update_time_entry_note(&*pool, params.id, query.note).await?;

    Ok(Json(entry.into()))
}

pub async fn play_time_entry_request(
    Path(params): Path<IdPath>,
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<TimeEntryVM>> {
    let entry = switch_to_timer(&pool, params.id).await?;
    Ok(Json(entry.into()))
}

pub async fn pause_time_entry_request(
    Path(params): Path<IdPath>,
    pool: Extension<sqlx::PgPool>,
) -> Result<Json<TimeEntryVM>> {
    let entry = fetch_time_entry_by_id(&*pool, params.id).await?;
    let paused_entry = pause_timer(&pool, entry).await?;
    Ok(Json(paused_entry.into()))
}

pub async fn delete_time_entry_request(
    Path(params): Path<IdPath>,
    pool: Extension<sqlx::PgPool>,
) -> Result<()> {
    delete_time_entry(&*pool, params.id).await?;
    Ok(())
}
