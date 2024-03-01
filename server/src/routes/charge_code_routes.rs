use crate::db::charge_code_repo::fetch_charge_codes;
use crate::utils::error::Result;
use axum::{Extension, Json};
use shared_lib::models::charge_code::ChargeCode;
use sqlx::SqlitePool;

pub async fn get_charge_codes(Extension(pool): Extension<SqlitePool>) -> Result<Json<Vec<ChargeCode>>> {
    let records = fetch_charge_codes(&pool).await?;

    Ok(Json(records))
}
