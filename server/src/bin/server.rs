use axum::{routing::get, Router};
use rustic_server::routes::{
    charge_code_routes::get_charge_codes, time_entry_routes::get_time_entries,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://rustic_user:password@localhost:5433/rustic_db".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to the database");

    let app = Router::new()
        .route("/time_entries", get(get_time_entries))
        .route("/charge_codes", get(get_charge_codes))
        .layer(axum::extract::Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
