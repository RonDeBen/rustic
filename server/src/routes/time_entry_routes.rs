use crate::db::charge_code_repo::fetch_charge_codes;
use crate::db::time_entry_repo::update_time_entry_note;
use crate::models::{DayTimeEntries, FullState};
use crate::services::time_entry_service::{pause_timer_and_get_entries, switch_to_timer};
use crate::utils::error::Result;
use crate::{
    db::time_entry_repo::*,
    models::time_entry::{Day, TimeEntryVM},
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use serde::Deserialize;
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

pub async fn get_time_entries_request(
    Query(day): Query<i16>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<TimeEntryVM>>> {
    let records = fetch_time_entries_for_day(&pool, day).await?;
    let vms: Vec<TimeEntryVM> = records.iter().map(|x| x.into()).collect();

    Ok(Json(vms))
}

pub async fn create_time_entry_request(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DayTimeEntries>> {
    let day = Day::get_current_day();
    let entry = create_time_entry(&pool, day).await?;
    let entries = fetch_time_entries_for_day(&pool, entry.day.into()).await?;

    let day_time_entries = DayTimeEntries::new(day, entries.as_slice());

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

pub async fn update_time_entry_time_request(
    Path(params): Path<EntryAndTimePath>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<TimeEntryVM>> {
    update_time_for_time_entry(&pool, params.id, params.total_time).await?;
    let updated_entry = fetch_time_entry_by_id(&pool, params.id).await?;
    Ok(Json(updated_entry.into()))
}

pub async fn update_time_entry_note_request(
    Path(id): Path<i32>,
    Query(note): Query<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<TimeEntryVM>> {
    println!("updating entry {}, with note: {}", id, note);
    update_time_entry_note(&pool, id, note).await?;
    let entry = fetch_time_entry_by_id(&pool, id).await?;
    println!("entry: {:?}", entry);

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
    let entries = pause_timer_and_get_entries(&pool, &entry).await?;
    Ok(Json(entries))
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
