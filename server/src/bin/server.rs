use axum::{extract::Query, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
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

async fn get_time_entries(
    Query(params): Query<TimeEntryQuery>,
    pool: axum::extract::Extension<sqlx::PgPool>,
) -> Json<Vec<TimeEntry>> {
    let records = sqlx::query_as::<_, TimeEntry>(
        "SELECT id, start_time, total_time, note, day FROM time_tracking.time_entries WHERE day = $1"
    )
    .bind(params.day)
    .fetch_all(&*pool)
    .await
    .unwrap();

    Json(records)
}

// async fn handler() -> &'static str {
//     "Does this still work?"
// }

#[derive(Deserialize)]
struct TimeEntryQuery {
    day: i32,
}

#[derive(Serialize, sqlx::FromRow)]
struct TimeEntry {
    id: i32,
    start_time: i64,
    total_time: f64,
    note: String,
    day: i32,
}
