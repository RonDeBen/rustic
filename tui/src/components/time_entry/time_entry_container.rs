use super::{entry::TimeEntry, time_utils::format_millis};
use crate::{
    action::{Action, TTAct, UIAct},
    api_client::ApiRequest::*,
    components::Component,
    tui::Frame,
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

pub struct TimeEntryContainer {
    entries: Vec<TimeEntry>,
    selected_index: usize,
    command_tx: Option<UnboundedSender<Action>>,
}

impl TimeEntryContainer {
    pub fn new(entries: Vec<TimeEntry>, selected_index: usize) -> Self {
        Self {
            entries,
            selected_index,
            command_tx: None,
        }
    }

    pub fn set_time_entries(&mut self, entries: Vec<TimeEntry>) {
        self.entries = entries;
    }

    pub fn get_selected_entry(&self) -> Option<&TimeEntry> {
        self.entries.get(self.selected_index)
    }

    fn calculate_total_millis(&self) -> i64 {
        self.entries.iter().map(|e| e.total_milliseconds()).sum()
    }

    fn total_elapsed_time_string(&self) -> String {
        let time_string = format_millis(&self.calculate_total_millis());
        format!("Total Time: {}", time_string)
    }
}

impl Component for TimeEntryContainer {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::UI(UIAct::Tick) => {
                for entry in &mut self.entries {
                    entry.update(action.clone())?;
                }
            }
            _ => { /* ... */ }
        }
        Ok(None)
    }
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if !self.entries.is_empty() {
                    self.selected_index =
                        (self.selected_index + self.entries.len() - 1) % self.entries.len();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.entries.is_empty() {
                    self.selected_index = (self.selected_index + 1) % self.entries.len();
                }
            }
            KeyCode::Char(' ') => {
                // Toggle the timer for the selected entry
                if let Some(entry) = self.entries.get_mut(self.selected_index) {
                    entry.is_active = !entry.is_active;
                    match entry.is_active {
                        // went from pause to play
                        true => {
                            if let Some(tx) = &self.command_tx {
                                tx.send(Action::api_request_action(PlayEntry { id: entry.id }))?;
                            }
                        }
                        // went from play to pause
                        false => {
                            if let Some(tx) = &self.command_tx {
                                tx.send(Action::api_request_action(PauseEntry { id: entry.id }))?;
                            }
                        }
                    }

                    // Stop other timers
                    for (i, e) in self.entries.iter_mut().enumerate() {
                        if i != self.selected_index {
                            e.is_active = false;
                            e.start_time = None;
                        }
                    }
                }
            }
            KeyCode::Char('a') => {
                if let Some(tx) = &self.command_tx {
                    tx.send(Action::api_request_action(CreateTimeEntry))?;
                }
            }
            KeyCode::Char('t') => {
                //TODO: modify the timer on this entry
            }
            KeyCode::Char('d') => {
                if let (Some(tx), Some(entry)) = (&self.command_tx, self.get_selected_entry()) {
                    tx.send(Action::api_request_action(DeleteEntry { id: entry.id }))?;
                }
            }
            KeyCode::Char('c') => {
                if let (Some(tx), Some(entry)) = (&self.command_tx, self.get_selected_entry()) {
                    tx.send(Action::TT(TTAct::EditChargeCode(entry.id)))?;
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let block = Block::default()
            .title(self.total_elapsed_time_string())
            .borders(Borders::ALL);
        f.render_widget(block, rect);

        let inner_area = rect.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        });
        let entry_height = 3; // Adjust this based on how much space you want each entry to take

        // Check if there are more entries than the area can fit, adjust layout accordingly
        let total_height_needed = self.entries.len() as u16 * entry_height;
        let scrollable = total_height_needed > inner_area.height;

        // Create a layout for the entries
        let constraints = if scrollable {
            vec![Constraint::Min(entry_height)]
        } else {
            self.entries
                .iter()
                .map(|_| Constraint::Length(entry_height))
                .collect()
        };

        let entry_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area);

        for (i, entry) in self.entries.iter_mut().enumerate() {
            if let Some(chunk) = entry_chunks.get(i) {
                entry.is_selected = i == self.selected_index;
                entry.draw(f, *chunk)?;
            }
        }

        Ok(())
    }
}
