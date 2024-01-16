use notify_rust::Notification;
// use shared_models::full_state::FullState;
use tokio::time::{sleep, Duration};

pub mod monitor_actions;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let check_interval = Duration::from_secs(600); // 10 minutes

    loop {
        sleep(check_interval).await;
    }
}

// pub async fn get_full_state(&self) -> Result<FullState, reqwest::Error> {
//     self.client
//         .get(&format!("{}/full_state", self.base_url))
//         .send()
//         .await?
//         .json::<FullState>()
//         .await
// }

