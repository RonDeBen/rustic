use axum::{
    routing::{delete, get, post, put},
    Router,
};
use rustic_server::{
    routes::{charge_code_routes::*, time_entry_routes::*},
    utils,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    env_logger::init();

    let pool = utils::connections::get_connection().await;
    utils::connections::init_db(&pool).await;

    let app = Router::new()
        .route("/full_state", get(get_everything_request))
        .route("/time_entries/day/:day", post(create_time_entry_request))
        .route(
            "/time_entries/:id/charge_code/:code_id",
            put(update_time_entry_charge_code_request),
        )
        .route(
            "/time_entries/:id/time/:total_time",
            put(update_time_entry_time_request),
        )
        .route(
            "/time_entries/:id/add_time/:add_time",
            put(add_time_to_entry_request),
        )
        .route(
            "/time_entries/:id/note",
            put(update_time_entry_note_request),
        )
        .route("/time_entries/:id/play", put(play_time_entry_request))
        .route("/time_entries/:id/pause", put(pause_time_entry_request))
        .route("/time_entries/:id", delete(delete_time_entry_request))
        .route("/time_entries/costpoint", get(get_costpoint_entries))
        .route("/time_entries/update", put(update_time_entry_request))
        .route("/charge_codes", get(get_charge_codes))
        .route("/admin/cleanup", post(delete_old_entries_request))
        .layer(axum::extract::Extension(pool))
        .layer(CorsLayer::permissive());

    let default_addr = "127.0.0.1:3000".to_string();
    let addr = std::env::var("SERVER_ADDR").unwrap_or(default_addr);
    let addr: SocketAddr = addr.parse().expect("Invalid address");

    log::info!("Starting server on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
