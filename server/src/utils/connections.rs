use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_connection() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://rustic_user:password@localhost:5433/rustic_db".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to the database")
}
