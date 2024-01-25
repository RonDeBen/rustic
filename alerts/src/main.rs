use notify_rust::Notification;
use shared_lib::{
    api_client::ApiClient,
    models::{day::Day, time_entry::TimeEntryVM},
};
use tokio::time::{sleep, Duration};

pub mod error;

use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let api_base_url =
        std::env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8001".to_string());
    let api_client = ApiClient::new(api_base_url);

    let interval_secs = std::env::var("CHECK_INTERVAL_SECS").unwrap_or("5".to_string()); // 5 minutes
    let check_interval = match interval_secs.parse::<u64>() {
        Ok(secs) => Duration::from_secs(secs),
        Err(_) => Duration::from_secs(5), // 5 minutes
    };

    let mut notification_sent = false;
    let mut last_notification_day: Option<Day> = None;

    log::info!("Starting monitor workday service.");

    loop {
        let current_day = Day::get_current_day();

        if last_notification_day.map_or(true, |last_day| last_day != current_day) {
            log::info!("A new day detected. Resetting notification flag.");
            notification_sent = false;
            last_notification_day = Some(current_day);
        }

        if !notification_sent && eod_check(&api_client).await? {
            log::info!("End-of-day notification sent for {:?}", current_day);
            notification_sent = true;
        }

        sleep(check_interval).await;
    }
}

async fn eod_check(client: &ApiClient) -> Result<bool> {
    let full_state = client.get_full_state().await?;

    let current_day = Day::get_current_day();

    let todays_entries = match full_state.get_vms_for_day(current_day) {
        Some(vms) => vms,
        None => return Ok(false),
    };

    let total_minutes = sum_to_nearest_quarter_hour(todays_entries.as_slice());

    // 7.5 hours worked
    match total_minutes >= 450 {
        true => {
            send_notification("You are close to 8 hours worked today!!")?;
            Ok(true)
        }
        false => Ok(false),
    }
}

fn sum_to_nearest_quarter_hour(entries: &[TimeEntryVM]) -> u16 {
    let total_time_millis: i64 = entries.iter().map(|entry| entry.real_total_time()).sum();
    let total_minutes = total_time_millis / 1000 / 60; // Convert milliseconds to minutes
    ((total_minutes as f64 / 15.0).round() * 15.0) as u16 // Round to nearest quarter hour
}

pub fn send_notification(message: &str) -> Result<()> {
    Notification::new()
        .summary("Rustic Notification")
        .body(message)
        .show()?;
    Ok(())
}
