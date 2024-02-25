use serde::{Deserialize, Serialize};
use shared_lib::models::day::Day;
use strum::Display;

use crate::{
    api_client::{ApiRequest, ApiResponse},
    mode::Mode,
};

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
    UpdateSelectedEntry,
    EditChargeCode(i32),
    EditTime(EditTimeAction),
    SwapTime(i32),
    UpdateMode(Mode),
    SaveState,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EditTimeAction {
    pub id: i32,
    pub millis: i64,
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

    pub fn edit_time_action(tt_act: EditTimeAction) -> Self {
        Action::TT(TTAct::EditTime(tt_act))
    }
}
