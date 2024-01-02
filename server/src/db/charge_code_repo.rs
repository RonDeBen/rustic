use sqlx::PgPool;

use crate::models::charge_code::ChargeCode;

pub async fn fetch_charge_codes(pool: &PgPool) -> Result<Vec<ChargeCode>, sqlx::Error> {
    sqlx::query_as::<_, ChargeCode>("SELECT id, alias, code, is_nc FROM time_tracking.charge_codes")
        .fetch_all(pool)
        .await
}
