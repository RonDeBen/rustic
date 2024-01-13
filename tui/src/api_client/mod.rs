pub mod models;

use self::models::{time_entry::TimeEntryVM, DayTimeEntries, FullState};
use crate::action::{Action, ApiAct, UIAct};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use strum::Display;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum ApiRequest {
    GetFullState,
    CreateTimeEntry,
    UpdateChargeCode {
        time_entry_id: i32,
        charge_code_id: i32,
    },
    UpdateEntryNote {
        id: i32,
        note: String,
    },
    PlayEntry {
        id: i32,
    },
    PauseEntry {
        id: i32,
    },
    DeleteEntry {
        id: i32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum ApiResponse {
    FullState(FullState),
    DayEntriesUpdate(DayTimeEntries),
    TimeEntryUpdate(TimeEntryVM),
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        ApiClient {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn process_api_action(&self, action: &ApiAct, action_tx: &UnboundedSender<Action>) {
        match self.process_api_action_inner(action, action_tx).await {
            Ok(_x) => {}
            // if we have reqwest errors, swallow them but send an error action
            Err(error) => action_tx
                .send(Action::UI(UIAct::Error(format!(
                    "Failed api request: {:?}",
                    error
                ))))
                .unwrap(),
        }
    }

    pub async fn process_api_action_inner(
        &self,
        action: &ApiAct,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<(), reqwest::Error> {
        match action {
            ApiAct::Request(request) => match request {
                ApiRequest::GetFullState => {
                    let rcv = self.get_full_state().await?;
                    let response = ApiResponse::FullState(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                }
                ApiRequest::CreateTimeEntry => {
                    let rcv = self.create_time_entry().await?;
                    let response = ApiResponse::DayEntriesUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                }
                ApiRequest::UpdateChargeCode {
                    time_entry_id,
                    charge_code_id,
                } => {
                    let rcv = self
                        .update_time_entry_charge_code(*time_entry_id, *charge_code_id)
                        .await?;
                    let response = ApiResponse::TimeEntryUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                }
                ApiRequest::UpdateEntryNote { id, note } => {
                    let rcv = self.update_entry_note(*id, note.to_owned()).await?;
                    let response = ApiResponse::TimeEntryUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                }
                ApiRequest::PlayEntry { id } => {
                    let rcv = self.play_entry(*id).await?;
                    let response = ApiResponse::DayEntriesUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                }
                ApiRequest::PauseEntry { id } => {
                    let rcv = self.pause_entry(*id).await?;
                    let response = ApiResponse::DayEntriesUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                }
                ApiRequest::DeleteEntry { id } => {
                    let rcv = self.delete_entry(*id).await?;
                    let response = ApiResponse::DayEntriesUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                }
            },
            ApiAct::Response(_response) => {
                // intentionally left empty
                // only handle the actions that want us to use the api_client here
                // handle responses in the UI
                Ok(())
            }
            ApiAct::Error(error) => {
                action_tx
                    .send(Action::UI(UIAct::Error(format!(
                        "Failed api request: {:?}",
                        error
                    ))))
                    .unwrap();
                Ok(())
            }
        }
    }

    pub async fn get_full_state(&self) -> Result<FullState, reqwest::Error> {
        self.client
            .get(&format!("{}/full_state", self.base_url))
            .send()
            .await?
            .json::<FullState>()
            .await
    }

    pub async fn create_time_entry(&self) -> Result<DayTimeEntries, reqwest::Error> {
        self.client
            .post(&format!("{}/time_entry", self.base_url))
            .send()
            .await?
            .json::<DayTimeEntries>()
            .await
    }

    pub async fn update_time_entry_charge_code(
        &self,
        time_entry_id: i32,
        charge_code_id: i32,
    ) -> Result<TimeEntryVM, reqwest::Error> {
        self.client
            .put(&format!(
                "{}/time_entry/{}/charge_code/{}",
                self.base_url, time_entry_id, charge_code_id
            ))
            .send()
            .await?
            .json::<TimeEntryVM>()
            .await
    }

    pub async fn update_entry_note(
        &self,
        id: i32,
        note: String,
    ) -> Result<TimeEntryVM, reqwest::Error> {
        self.client
            .put(&format!("{}/time_entry/{}", self.base_url, id))
            .query(&[("note", note)])
            .send()
            .await?
            .json::<TimeEntryVM>()
            .await
    }

    pub async fn play_entry(&self, id: i32) -> Result<DayTimeEntries, reqwest::Error> {
        self.client
            .put(&format!("{}/time_entry/play/{}", self.base_url, id))
            .send()
            .await?
            .json::<DayTimeEntries>()
            .await
    }

    pub async fn pause_entry(&self, id: i32) -> Result<DayTimeEntries, reqwest::Error> {
        self.client
            .put(&format!("{}/time_entry/pause/{}", self.base_url, id))
            .send()
            .await?
            .json::<DayTimeEntries>()
            .await
    }

    pub async fn delete_entry(&self, id: i32) -> Result<DayTimeEntries, reqwest::Error> {
        self.client
            .delete(&format!("{}/time_entry/{}", self.base_url, id))
            .send()
            .await?
            .json::<DayTimeEntries>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::api_client::ApiClient;

    #[tokio::test]
    async fn can_create_time_entry() {
        let api_base_url =
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let api_client = ApiClient::new(api_base_url);

        // Call the function under test
        let result = api_client.create_time_entry().await;

        // Check the result and print the error if it exists
        match result {
            Ok(time_entry) => {
                // If it's okay, you can optionally print the time entry or perform further checks
                println!("Success: {:?}", time_entry);
            }
            Err(e) => {
                // Print the error and assert false to make sure the test fails
                eprintln!("Error occurred: {:?}", e);
                panic!("unwrapped an error");
            }
        }
    }
}
