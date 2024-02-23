use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::{
    full_state::{DayTimeEntries, FullState},
    time_entry::TimeEntryVM,
};

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

#[derive(Serialize, Deserialize)]
struct NotePaylaod {
    note: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        ApiClient {
            client: Client::new(),
            base_url,
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

    pub async fn create_time_entry(&self, day: i16) -> Result<DayTimeEntries, reqwest::Error> {
        self.client
            .post(&format!("{}/time_entries/day/{}", self.base_url, day))
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
                "{}/time_entries/{}/charge_code/{}",
                self.base_url, time_entry_id, charge_code_id
            ))
            .send()
            .await?
            .json::<TimeEntryVM>()
            .await
    }

    pub async fn update_time_entry_time(
        &self,
        time_entry_id: i32,
        total_time: i64,
    ) -> Result<TimeEntryVM, reqwest::Error> {
        self.client
            .put(&format!(
                "{}/time_entries/{}/time/{}",
                self.base_url, time_entry_id, total_time
            ))
            .send()
            .await?
            .json::<TimeEntryVM>()
            .await
    }

    pub async fn add_time_to_entry(
        &self,
        time_entry_id: i32,
        add_time: i64,
    ) -> Result<TimeEntryVM, reqwest::Error> {
        self.client
            .put(&format!(
                "{}/time_entries/{}/add_time/{}",
                self.base_url, time_entry_id, add_time
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
            .put(&format!("{}/time_entries/{}/note", self.base_url, id))
            .json(&NotePaylaod { note })
            .send()
            .await?
            .json::<TimeEntryVM>()
            .await
    }

    pub async fn play_entry(&self, id: i32) -> Result<DayTimeEntries, reqwest::Error> {
        self.client
            .put(&format!("{}/time_entries/{}/play", self.base_url, id))
            .send()
            .await?
            .json::<DayTimeEntries>()
            .await
    }

    pub async fn pause_entry(&self, id: i32) -> Result<DayTimeEntries, reqwest::Error> {
        self.client
            .put(&format!("{}/time_entries/{}/pause", self.base_url, id))
            .send()
            .await?
            .json::<DayTimeEntries>()
            .await
    }

    pub async fn delete_entry(&self, id: i32) -> Result<DayTimeEntries, reqwest::Error> {
        self.client
            .delete(&format!("{}/time_entries/{}", self.base_url, id))
            .send()
            .await?
            .json::<DayTimeEntries>()
            .await
    }

    pub async fn cleanup_entries(&self) -> Result<(), reqwest::Error> {
        self.client
            .post(&format!("{}/admin/cleanup", self.base_url))
            .send()
            .await?
            .error_for_status()
            .map(|_| ())
    }
}
