use axum::{
    routing::{delete, get, post, put},
    Router,
};
use rustic_server::routes::{charge_code_routes::*, time_entry_routes::*};
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
        .route("/time_entries_by_day", get(get_time_entries_by_day))
        .route("/time_entries", get(get_time_entries_request))
        .route("/time_entry", post(create_time_entry_request))
        .route("/time_entry/:id", put(update_time_entry_note_request))
        .route("/time_entry/play/:id", put(play_time_entry_request))
        .route("/time_entry/pause/:id", put(pause_time_entry_request))
        .route("/time_entry/:id", delete(delete_time_entry_request))
        .route("/charge_code", get(get_charge_codes))
        .layer(axum::extract::Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
