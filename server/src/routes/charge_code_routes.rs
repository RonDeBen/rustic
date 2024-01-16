use crate::db::charge_code_repo::fetch_charge_codes;
use crate::utils::error::Result;
use axum::{Extension, Json};
use shared_models::charge_code::ChargeCode;
use sqlx::PgPool;

pub async fn get_charge_codes(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<ChargeCode>>> {
    let records = fetch_charge_codes(&pool).await?;

    Ok(Json(records))
}
