use crate::{db::charge_code_repo::fetch_charge_codes, models::charge_code::ChargeCode};
use axum::Json;
use crate::utils::error::Result;

pub async fn get_charge_codes(
    pool: axum::extract::Extension<sqlx::PgPool>,
) -> Result<Json<Vec<ChargeCode>>> {
    let records = fetch_charge_codes(&pool).await?;

    Ok(Json(records))
}
