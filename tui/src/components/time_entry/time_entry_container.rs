use crate::{action::Action, components::Component, tui::Frame};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use super::entry::TimeEntry;

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
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected_index = (self.selected_index - 1) % self.entries.len();
                // if self.selected_index > 0 {
                //     self.selected_index -= 1;
                // }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_index = (self.selected_index + 1) % self.entries.len();
                // if self.selected_index < self.entries.len() - 1 {
                //     self.selected_index += 1;
                // }
            }
            KeyCode::Char(' ') => {
                // Toggle the timer for the selected entry
                if let Some(entry) = self.entries.get_mut(self.selected_index) {
                    entry.is_active = !entry.is_active;

                    // Stop other timers
                    for (i, e) in self.entries.iter_mut().enumerate() {
                        if i != self.selected_index {
                            e.is_active = false;
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let block = Block::default().title("Time Entries").borders(Borders::ALL);
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
