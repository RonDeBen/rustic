use crate::utils::error::{MonitorError, Result};
use async_recursion::async_recursion;
use notify_rust::Notification;
use shared_lib::{api_client::ApiClient, models::full_state::FullState};

use super::{MonitorAction, MonitorActionResult};

pub struct MonitorOrchestrator {
    actions: Vec<Box<dyn MonitorAction>>,
    client: ApiClient,
}

impl MonitorOrchestrator {
    pub fn new(client: ApiClient) -> Self {
        Self {
            actions: Vec::new(),
            client,
        }
    }

    pub fn add_action<A: MonitorAction + 'static>(&mut self, action: A) {
        self.actions.push(Box::new(action));
    }

    pub async fn monitor_actions(&self) -> Result<()> {
        log::info!("running all monitor actions");
        self.cleanup_old_timers().await?;

        let full_state = self.get_full_state().await?;
        let results = self.run(&full_state);
        for result in results {
            handle_monitor_action_result(&self.client, result).await?;
        }
        Ok(())
    }

    fn run(&self, full_state: &FullState) -> Vec<MonitorActionResult> {
        self.actions
            .iter()
            .filter_map(|action| action.execute(full_state))
            .collect()
    }

    async fn get_full_state(&self) -> Result<FullState> {
        match self.client.get_full_state().await {
            Ok(state) => Ok(state),
            Err(e) => Err(MonitorError::ReqwestError(e)),
        }
    }

    async fn cleanup_old_timers(&self) -> Result<()> {
        match self.client.cleanup_entries().await {
            Ok(state) => Ok(state),
            Err(e) => Err(MonitorError::ReqwestError(e)),
        }
    }
}

#[async_recursion]
async fn handle_monitor_action_result(
    client: &ApiClient,
    result: MonitorActionResult,
) -> Result<()> {
    match result {
        MonitorActionResult::SendMessage(message) => {
            send_notification(&message)?;
            Ok(())
        }
        MonitorActionResult::StopTimer(entry_id) => {
            client.pause_entry(entry_id).await?;
            Ok(())
        }
        MonitorActionResult::DeleteEntry(entry_id) => {
            client.delete_entry(entry_id).await?;
            Ok(())
        }
        MonitorActionResult::Multiple(results) => {
            for sub_result in results {
                handle_monitor_action_result(client, sub_result).await?
            }
            Ok(())
        }
    }
}

fn send_notification(message: &str) -> Result<()> {
    Notification::new()
        .summary("Rustic Notification")
        .body(message)
        .show()?;
    Ok(())
}
