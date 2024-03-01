use std::path::PathBuf;

use sqlx::{
    migrate, sqlite::SqliteConnectOptions, Pool, Sqlite
};

pub async fn get_connection() -> Pool<Sqlite> {
    let database_file = 
    std::env::var("SQLITE_FILE").ok()
        .map(|s| PathBuf::from(s))
        .or(
            dirs::data_dir().map(|d| d.join("rustic/time_tracking.db"))
        )
    .expect("Failed to determine database location");

    Pool::connect_with(SqliteConnectOptions::new()
        .filename(database_file)
        .create_if_missing(true)
    )
        .await
        .expect("Failed to connect to sqlite")
}

pub async fn init_db(pool: &Pool<Sqlite>) {
    migrate!().run(pool).await.expect("Failed to initialize database");
}
