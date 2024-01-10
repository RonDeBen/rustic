use crate::utils::error::Result;
use crate::{db::charge_code_repo::fetch_charge_codes, models::charge_code::ChargeCode};
use axum::{Json, Extension};
use sqlx::PgPool;

pub async fn get_charge_codes(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<ChargeCode>>> {
    let records = fetch_charge_codes(&pool).await?;

    Ok(Json(records))
}
