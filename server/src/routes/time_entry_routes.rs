use crate::db::charge_code_repo::fetch_charge_codes;
use crate::db::time_entry_repo::update_time_entry_note;
use crate::db::time_entry_repo::*;
use crate::models::costpoint_entry::CostpointEntryVM;
use crate::models::DayTimeEntries;
use crate::services::time_entry_service::switch_to_timer;
use crate::utils::error::Result;
use crate::utils::time::get_elapsed_time;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use shared_lib::models::{full_state::FullState, time_entry::TimeEntryVM};
use sqlx::PgPool;

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

pub async fn create_time_entry_request(
    Path(day_num): Path<i16>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let _ = create_time_entry(&pool, day_num.into()).await?;
    let entries = fetch_time_entries_for_day(&pool, day_num).await?;

    let day_time_entries = DayTimeEntries::new(day_num.into(), entries.as_slice());

    Ok(Json(day_time_entries))
}

#[derive(Deserialize)]
pub struct EntryAndCodeIdPath {
    id: i32,
    code_id: i32,
}

pub async fn update_time_entry_charge_code_request(
    Path(params): Path<EntryAndCodeIdPath>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<TimeEntryVM>> {
    update_charge_code_for_time_entry(&pool, params.id, params.code_id).await?;
    let updated_entry = fetch_time_entry_by_id(&pool, params.id).await?;
    Ok(Json(updated_entry.into()))
}

#[derive(Deserialize)]
pub struct EntryAndTimePath {
    id: i32,
    total_time: i64,
}

pub async fn add_time_to_entry_request(
    Path((id, add_time)): Path<(i32, i64)>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<TimeEntryVM>> {
    add_time_to_entry(&pool, id, add_time).await?;
    let updated_entry = fetch_time_entry_by_id(&pool, id).await?;
    Ok(Json(updated_entry.into()))
}

pub async fn update_time_entry_time_request(
    Path(params): Path<EntryAndTimePath>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<TimeEntryVM>> {
    update_time_for_time_entry(&pool, params.id, params.total_time).await?;
    let updated_entry = fetch_time_entry_by_id(&pool, params.id).await?;
    Ok(Json(updated_entry.into()))
}

#[derive(Deserialize, Serialize)]
pub struct NotePayload {
    pub note: String,
}

pub async fn update_time_entry_note_request(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(body): Json<NotePayload>,
) -> Result<Json<TimeEntryVM>> {
    update_time_entry_note(&pool, id, body.note).await?;
    let entry = fetch_time_entry_by_id(&pool, id).await?;

    Ok(Json(entry.into()))
}

pub async fn play_time_entry_request(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let entries = switch_to_timer(&pool, id).await?;
    Ok(Json(entries))
}

pub async fn pause_time_entry_request(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let entry = fetch_time_entry_by_id(&pool, id).await?;
    let elapsed_time = get_elapsed_time(&entry);
    pause_time_entry(&pool, id, elapsed_time).await?;

    let entries = fetch_time_entries_for_day(&pool, entry.day.into()).await?;
    let day_entries = DayTimeEntries::new(entry.day, entries.as_slice());

    Ok(Json(day_entries))
}

pub async fn delete_time_entry_request(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let entry = fetch_time_entry_by_id(&pool, id).await?;
    delete_time_entry(&pool, id).await?;
    let entries = fetch_time_entries_for_day(&pool, entry.day.into()).await?;
    let day_time_entries = DayTimeEntries::new(entry.day, entries.as_slice());
    Ok(Json(day_time_entries))
}

pub async fn delete_old_entries_request(Extension(pool): Extension<PgPool>) -> Result<StatusCode> {
    delete_old_time_entries(&pool).await?;
    Ok(StatusCode::OK)
}

pub async fn get_costpoint_entries(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<CostpointEntryVM>>> {
    let mut raw_entries = fetch_costpoint_entries(&pool).await?;
    let entries: Vec<CostpointEntryVM> = raw_entries.drain(..).map(|entry| entry.into()).collect();

    Ok(Json(entries))
}
