use notify_rust::Notification;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let check_interval = Duration::from_secs(600); // 10 minutes

    loop {
        if check_work_hours().await {
            send_notification()?;
        };
        sleep(check_interval).await;
    }
}

async fn check_work_hours() -> bool {
    //TODO:
    // Use reqwest to make a request to your time tracking service
    // Check if the work hours are close to 8 hours
    // If yes, trigger the notification
    false
}

fn send_notification() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("Work Notification")
        .body("You are close to 8 hours of work today.")
        .show()?;
    Ok(())
}
