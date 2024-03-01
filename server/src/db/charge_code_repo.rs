use shared_lib::models::charge_code::ChargeCode;
use sqlx::SqlitePool;

pub async fn fetch_charge_codes(pool: &SqlitePool) -> Result<Vec<ChargeCode>, sqlx::Error> {
    sqlx::query_as::<_, ChargeCode>("SELECT id, alias, code, is_nc FROM charge_codes")
        .fetch_all(pool)
        .await
}
