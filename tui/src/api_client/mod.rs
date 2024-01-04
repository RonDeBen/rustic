pub mod models;

use reqwest::Client;

use self::models::{time_entry::TimeEntryVM, FullState};

pub struct ApiClient {
    client: Client,
    base_url: String,
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

    pub async fn create_time_entry(&self) -> Result<TimeEntryVM, reqwest::Error> {
        self.client
            .post(&format!("{}/time_entry", self.base_url))
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

    pub async fn play_entry(&self, id: i32) -> Result<TimeEntryVM, reqwest::Error> {
        self.client
            .put(&format!("{}/time_entry/play/{}", self.base_url, id))
            .send()
            .await?
            .json::<TimeEntryVM>()
            .await
    }

    pub async fn pause_entry(&self, id: i32) -> Result<TimeEntryVM, reqwest::Error> {
        self.client
            .put(&format!("{}/time_entry/pause/{}", self.base_url, id))
            .send()
            .await?
            .json::<TimeEntryVM>()
            .await
    }

    pub async fn delete_entry(&self, id: i32) -> Result<(), reqwest::Error> {
        let response = self
            .client
            .delete(&format!("{}/time_entry/{}", self.base_url, id))
            .send()
            .await?;

        match response.error_for_status() {
            Ok(_res) => Ok(()),
            Err(err) => {
                Err(err)
            }
        }
    }
}
