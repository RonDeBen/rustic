use monitor_actions::{
    actions::{
        long_running_timer_check::LongTimerCheck, midnight_check::MidnightTimerCheck,
        nearing_eod_check::EodCheck,
    },
    monitor_orchistrator::MonitorOrchestrator,
};
use shared_lib::api_client::ApiClient;
use tokio::time::{sleep, Duration};

pub mod monitor_actions;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let api_base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let api_client = ApiClient::new(api_base_url);

    let mut orchestrator = MonitorOrchestrator::new(api_client);
    orchestrator.add_action(LongTimerCheck {});
    orchestrator.add_action(EodCheck {});
    orchestrator.add_action(MidnightTimerCheck {});

    let check_interval = Duration::from_secs(600); // 10 minutes

    loop {
        sleep(check_interval).await;

        if let Err(e) = orchestrator.monitor_actions().await {
            log::error!("Error while running monitor actions: {}", e);
        }
    }
}
