use axum::{
    routing::{delete, get, post, put},
    Router,
};
use rustic_server::{
    routes::{charge_code_routes::*, time_entry_routes::*},
    utils,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = utils::connections::get_connection().await;

    let app = Router::new()
        .route("/full_state", get(get_everything_request))
        // .route("/time_entries_by_day", get(get_time_entries_by_day_request))
        .route("/time_entries", get(get_time_entries_request))
        .route("/time_entry", post(create_time_entry_request))
        .route("/time_entry/:id", put(update_time_entry_note_request))
        .route("/time_entry/play/:id", put(play_time_entry_request))
        .route("/time_entry/pause/:id", put(pause_time_entry_request))
        .route("/time_entry/:id", delete(delete_time_entry_request))
        .route("/charge_codes", get(get_charge_codes))
        .layer(axum::extract::Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
