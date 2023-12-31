use super::Component;
use crate::{action::Action, tui::Frame};
use chrono::Duration;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone, PartialEq)]
pub struct TimeEntry {
    charge_code: String,
    elapsed_time: Duration,
    is_active: bool,
}

impl Default for TimeEntry {
    fn default() -> Self {
        Self {
            charge_code: "".to_string(),
            elapsed_time: Duration::zero(),
            is_active: false,
        }
    }
}

impl TimeEntry {
    pub fn new() -> Self {
        Self::default()
    }
}


impl Component for TimeEntry {
    // maybe we don't need to handle key events here??
    // everything could be handled in the container
    //
    //fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    //    match key.code {
    //        //TODO: change selection with vim/arrow keys
    //        _ => {}
    //    };
    //    Ok(None)
    //}

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        Ok(())
    }
}
