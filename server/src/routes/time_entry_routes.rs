use crate::db::charge_code_repo::fetch_charge_codes;
use crate::db::time_entry_repo::update_time_entry_note;
use crate::models::{DayTimeEntries, FullState};
use crate::services::time_entry_service::{pause_timer_and_get_entries, switch_to_timer};
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
use sqlx::PgPool;
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

pub async fn get_everything_request(Extension(pool): Extension<PgPool>) -> Result<Json<FullState>> {
    let entries = fetch_all_time_entries(&pool).await?;
    let time_entries = organize_time_entries_by_day(entries);
    let charge_codes = fetch_charge_codes(&pool).await?;

    let full_state = FullState {
        time_entries,
        charge_codes,
    };

    Ok(Json(full_state))
}

pub async fn get_time_entries_by_day_request(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<HashMap<Day, Vec<TimeEntryVM>>>> {
    let entries = fetch_all_time_entries(&pool).await?;
    let organized_entries = organize_time_entries_by_day(entries);

    Ok(Json(organized_entries))
}

pub async fn get_time_entries_request(
    Query(params): Query<DayQuery>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<TimeEntryVM>>> {
    let records = fetch_time_entries_for_day(&pool, params.day).await?;
    let vms: Vec<TimeEntryVM> = records.iter().map(|x| x.into()).collect();

    Ok(Json(vms))
}

pub async fn create_time_entry_request(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let day = Day::get_current_day().ok_or(AppError::WeekendError)?;
    let entry = create_time_entry(&pool, day).await?;
    let entries = fetch_time_entries_for_day(&pool, entry.day.into()).await?;

    let day_time_entries = DayTimeEntries::new(day, entries.as_slice());

    Ok(Json(day_time_entries))
}

pub async fn update_time_entry_note_request(
    Path(params): Path<IdPath>,
    Query(query): Query<NoteQuery>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<TimeEntryVM>> {
    let entry = update_time_entry_note(&pool, params.id, query.note).await?;

    Ok(Json(entry.into()))
}

pub async fn play_time_entry_request(
    Path(params): Path<IdPath>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let entries = switch_to_timer(&pool, params.id).await?;
    let print_entries: Vec<i32> = entries.entries.iter().map(|x| x.id).collect();
    println!("in request: {:?}", print_entries);
    Ok(Json(entries))
}

pub async fn pause_time_entry_request(
    Path(params): Path<IdPath>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let entry = fetch_time_entry_by_id(&pool, params.id).await?;
    let entries = pause_timer_and_get_entries(&pool, &entry).await?;
    Ok(Json(entries))
}

pub async fn delete_time_entry_request(
    Path(params): Path<IdPath>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let entry = fetch_time_entry_by_id(&pool, params.id).await?;
    delete_time_entry(&pool, params.id).await?;
    let entries = fetch_time_entries_for_day(&pool, entry.day.into()).await?;
    let day_time_entries = DayTimeEntries::new(entry.day, entries.as_slice());
    Ok(Json(day_time_entries))
}
