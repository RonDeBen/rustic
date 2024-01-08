use serde::{Deserialize, Serialize};
use strum::Display;

use crate::api_client::{models::day::Day, ApiRequest, ApiResponse};

#[derive(Clone, PartialEq, Eq, Serialize, Display, Deserialize, Debug)]
pub enum Action {
    UI(UIAct),
    TT(TTAct),
    Api(ApiAct),
}

#[derive(Clone, PartialEq, Eq, Serialize, Display, Deserialize, Debug)]
pub enum UIAct {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
}

#[derive(Clone, PartialEq, Eq, Serialize, Display, Deserialize, Debug)]
pub enum TTAct {
    ChangeDay(Day),
    UpdateNote(String),
}

#[derive(Clone, PartialEq, Eq, Serialize, Display, Deserialize, Debug)]
pub enum ApiAct {
    Request(ApiRequest),
    Response(ApiResponse),
    Error(String),
}

impl Action {
    pub fn api_response_action(response: ApiResponse) -> Self {
        Action::Api(ApiAct::Response(response))
    }

    pub fn api_request_action(request: ApiRequest) -> Self {
        Action::Api(ApiAct::Request(request))
    }
}

