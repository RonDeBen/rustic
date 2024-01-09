use crate::models::charge_code::ChargeCode;
use sqlx::PgConnection;

pub async fn fetch_charge_codes(pool: &mut PgConnection) -> Result<Vec<ChargeCode>, sqlx::Error> {
    sqlx::query_as::<_, ChargeCode>("SELECT id, alias, code, is_nc FROM time_tracking.charge_codes")
        .fetch_all(&mut *pool)
        .await
}
