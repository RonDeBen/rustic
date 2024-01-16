use crate::utils::error::Result;
use async_recursion::async_recursion;
use notify_rust::Notification;
use shared_models::full_state::FullState;

use super::{MonitorAction, MonitorActionResult};

struct MonitorOrchestrator {
    actions: Vec<Box<dyn MonitorAction>>,
}

impl MonitorOrchestrator {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add_action<A: MonitorAction + 'static>(&mut self, action: A) {
        self.actions.push(Box::new(action));
    }

    pub fn run(&self, full_state: &FullState) -> Vec<MonitorActionResult> {
        self.actions
            .iter()
            .filter_map(|action| action.execute(full_state))
            .collect()
    }
}

#[async_recursion]
async fn handle_monitor_action_result(result: MonitorActionResult)-> Result<()> {
    match result {
        MonitorActionResult::SendMessage(message) => {
            send_notification(&message)?;
            Ok(())
        }
        MonitorActionResult::StopTimer(timer_id) => {
            // logic to stop the timer
            Ok(())
        }
        MonitorActionResult::DeleteEntry(entry_id) => {
            // logic to delete the entry
            Ok(())
        }
        MonitorActionResult::Multiple(results) => {
            for sub_result in results {
               handle_monitor_action_result(sub_result).await?
            }
            Ok(())
        } // ... handle other cases
    }
}

fn send_notification(message: &str) -> Result<()> {
    Notification::new()
        .summary("Rustic Notification")
        .body(message)
        .show()?;
    Ok(())
}
