use serde::{Deserialize, Serialize};
use strum::Display;

use crate::api_client::models::day::Day;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
    UI(UIAct),
    TT(TTAct),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum UIAct{
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum TTAct {
    ChangeDay(Day),
    UpdateNote(String),
}
