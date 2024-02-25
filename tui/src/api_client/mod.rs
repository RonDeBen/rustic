pub mod models;

use crate::action::{Action, ApiAct, UIAct};
use serde::{Deserialize, Serialize};
use shared_lib::{
    api_client::ApiClient,
    models::{
        full_state::{DayTimeEntries, FullState},
        time_entry::TimeEntryVM,
    },
};
use strum::Display;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum ApiRequest {
    GetFullState,
    CreateTimeEntry {
        day: i16,
    },
    UpdateChargeCode {
        time_entry_id: i32,
        charge_code_id: i32,
    },
    SetTime {
        id: i32,
        millis: i64,
    },
    AddTime {
        id: i32,
        millis: i64,
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
    FullEntryUpdate {
        entry: TimeEntryVM
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum ApiResponse {
    FullState(FullState),
    DayEntriesUpdate(DayTimeEntries),
    TimeEntryUpdate(TimeEntryVM),
}

#[derive(Serialize, Deserialize)]
struct NotePaylaod {
    note: String,
}

pub trait ApiClientExt {
    // public async traits is a no no, normally
    // these should only be used in this crate, so ignore warnings
    #[allow(async_fn_in_trait)]
    async fn process_api_action(&self, action: &ApiAct, action_tx: &UnboundedSender<Action>);
    #[allow(async_fn_in_trait)]
    async fn process_api_action_inner(
        &self,
        action: &ApiAct,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<(), reqwest::Error>;
}

impl ApiClientExt for ApiClient {
    async fn process_api_action(&self, action: &ApiAct, action_tx: &UnboundedSender<Action>) {
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

    async fn process_api_action_inner(
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
                ApiRequest::CreateTimeEntry { day } => {
                    let rcv = self.create_time_entry(*day).await?;
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
                ApiRequest::SetTime { id, millis } => {
                    let rcv = self.update_time_entry_time(*id, *millis).await?;
                    let response = ApiResponse::TimeEntryUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                }
                ApiRequest::AddTime { id, millis } => {
                    let rcv = self.add_time_to_entry(*id, *millis).await?;
                    let response = ApiResponse::TimeEntryUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                },
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
                ApiRequest::FullEntryUpdate { entry } => {
                    let rcv = self.update_time_entry(entry.to_owned()).await?;
                    let response = ApiResponse::DayEntriesUpdate(rcv);
                    action_tx
                        .send(Action::api_response_action(response))
                        .unwrap();
                    Ok(())
                },
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
}

