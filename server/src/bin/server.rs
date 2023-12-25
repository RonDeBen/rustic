use axum::{routing::get, Router};
use rustic_server::routes::time_entry_routes::get_time_entries;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rustic_user:password@db:5432/rustic_db")
        .await
        .expect("Could not connect to the database");

    let app = Router::new()
        .route("/time_entries", get(get_time_entries))
        .layer(axum::extract::Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
