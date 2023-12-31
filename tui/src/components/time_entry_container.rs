use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use super::Component;
use crate::{action::Action, components::time_entry::TimeEntry, tui::Frame};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TimeEntryContainer {
    entries: Vec<TimeEntry>,
    selected_index: usize,
}

impl TimeEntryContainer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for TimeEntryContainer {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        // match key.code {
        //     _ => {}
        // };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let block =Block::default()
            .title("TODO: Time Entry Container")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        f.render_widget(block, rect);

        Ok(())
    }
}
