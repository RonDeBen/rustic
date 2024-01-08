use crate::{
    db::time_entry_repo::{fetch_all_running_timers, pause_time_entry, play_time_entry},
    models::time_entry::TimeEntryRaw,
};
use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;

pub async fn switch_to_timer(pool: &PgPool, id: i32) -> Result<TimeEntryRaw, sqlx::Error> {
    // pause all running timer
    let running_timers = fetch_all_running_timers(pool).await?;
    for timer in running_timers {
        pause_timer(pool, timer).await?;
    }

    // start new timer
    let start_time: NaiveDateTime = Utc::now().naive_utc();
    play_time_entry(pool, id, start_time).await
}

pub async fn pause_timer(pool: &PgPool, entry: TimeEntryRaw) -> Result<TimeEntryRaw, sqlx::Error> {
    let elapsed_time = match entry.start_time {
        Some(start_time) => {
            let end_time: NaiveDateTime = Utc::now().naive_utc();
            (end_time - start_time).num_milliseconds()
        }
        None => 0, // timer was "played" before it was "paused"
    };
    pause_time_entry(pool, entry.id, elapsed_time).await
}

// #[cfg(test)]
// mod tests {
// use crate::db::time_entry_repo::*;
// use crate::models::time_entry::Day;
// use crate::services::time_entry_service::*;
// use crate::utils::connections::get_connection;
// use chrono::NaiveDateTime;
// use chrono::Utc;
// use std::time::Duration;

//     #[tokio::test]
//     async fn pausing_correctly_calculates_elapsed_time() {
//         let pool = get_connection().await;
//         let mut tx = pool.begin().await.unwrap();

//         let entry = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
//         let start_time: NaiveDateTime = Utc::now().naive_utc();
//         let played_entry = play_time_entry(&mut *tx, entry.id, start_time)
//             .await
//             .unwrap();

//         tokio::time::sleep(Duration::from_secs(1)).await;
//         let paused_entry = pause_timer(&mut *tx, played_entry).await.unwrap();

//         // is around 1 second
//         assert!(paused_entry.total_time >= 995);
//         assert!(paused_entry.total_time < 1005);

//         tx.rollback().await.unwrap()
//     }

//     #[tokio::test]
//     async fn starting_a_timer_pauses_other_timers() {
//         let pool = get_connection().await;
//         let mut tx = pool.begin().await.unwrap();

//         let entry1 = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
//         let entry2 = create_time_entry(&mut *tx, Day::Monday).await.unwrap();
//         let start_time: NaiveDateTime = Utc::now().naive_utc();
//         let _started_entry1 = play_time_entry(&mut *tx, entry1.id, start_time)
//             .await
//             .unwrap();

//         let swapped_to_entry2 = switch_to_timer(&mut *tx, entry2.id).await.unwrap();

//         let running_timers = fetch_all_running_timers(&mut *tx).await.unwrap();

//         assert_eq!(running_timers.len(), 1);
//         assert_eq!(running_timers.first().unwrap().id, swapped_to_entry2.id);

//         tx.rollback().await.unwrap()
//     }
// }
