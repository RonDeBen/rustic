use std::path::PathBuf;

use sqlx::{
    migrate, sqlite::SqliteConnectOptions, Pool, Sqlite
};
use tokio::fs;

pub async fn get_connection() -> Pool<Sqlite> {

    let data_dir = std::env::var("DATA_DIR").ok()
        .map(|s| PathBuf::from(s))
        .or(dirs::data_dir().map(|dir| dir.join("rustic")))
        .expect("Failed to determine data directory");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).await.expect("Failed to create data directory");
    }

    Pool::connect_with(SqliteConnectOptions::new()
        .filename(data_dir.join("time_tracking.db"))
        .create_if_missing(true)
    )
        .await
        .expect("Failed to connect to sqlite")
}

pub async fn init_db(pool: &Pool<Sqlite>) {
    migrate!().run(pool).await.expect("Failed to initialize database");
}
