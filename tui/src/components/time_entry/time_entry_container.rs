use super::{entry::TimeEntry, time_utils::format_millis};
use crate::{
    action::{Action, EditTimeAction, TTAct, UIAct},
    api_client::{models::day::Day, ApiRequest::*},
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
    scroll_position: usize,
    rect: Rect,
    current_day: Day,
}

impl TimeEntryContainer {
    pub fn new(entries: Vec<TimeEntry>, selected_index: usize, current_day: Day) -> Self {
        Self {
            entries,
            selected_index,
            command_tx: None,
            scroll_position: 0,
            rect: Rect::default(),
            current_day,
        }
    }

    pub fn set_time_entries(&mut self, entries: Vec<TimeEntry>) {
        self.entries = entries;
    }

    pub fn set_index(&mut self, index: usize) {
        self.selected_index = index;
    }

    pub fn set_day(&mut self, day: Day) {
        self.current_day = day;
    }

    pub fn get_selected_entry(&self) -> Option<TimeEntry> {
        self.entries.get(self.selected_index).cloned()
    }

    fn calculate_total_millis(&self) -> i64 {
        self.entries.iter().map(|e| e.total_milliseconds()).sum()
    }

    fn total_elapsed_time_string(&self) -> String {
        let time_string = format_millis(&self.calculate_total_millis());
        format!("Total Time: {}", time_string)
    }

    pub fn send_index_action(&mut self) {
        if let Some(tx) = &self.command_tx {
            tx.send(Action::TT(TTAct::UpdateSelectedEntry)).unwrap();
        }
    }

    fn adjust_scroll_position(&mut self, num_visible_entries: usize) {
        let num_entries = self.entries.len();
        if num_entries <= num_visible_entries {
            self.scroll_position = 0;
            return;
        }
        self.scroll_position = if self.selected_index >= num_visible_entries / 2 {
            std::cmp::min(
                self.selected_index - num_visible_entries / 2,
                num_entries - num_visible_entries,
            )
        } else {
            0
        };
    }

    fn calculate_num_visible_entries(&self) -> usize {
        let entry_height = 3; // Assuming each entry takes 3 lines
        self.rect.height as usize / entry_height
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
        let num_visible_entries = self.calculate_num_visible_entries();
        let num_entries = self.entries.len();

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if !self.entries.is_empty() {
                    self.selected_index = if self.selected_index == 0 {
                        num_entries - 1 // Wrap to the end
                    } else {
                        self.selected_index - 1
                    };
                    self.adjust_scroll_position(num_visible_entries);
                    self.send_index_action();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.entries.is_empty() {
                    self.selected_index = (self.selected_index + 1) % num_entries; // Wrap to the beginning
                    self.adjust_scroll_position(num_visible_entries);
                    self.send_index_action();
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
                    tx.send(Action::api_request_action(CreateTimeEntry {
                        day: self.current_day.into(),
                    }))?;
                }
            }
            KeyCode::Char('t') => {
                if let (Some(tx), Some(entry)) = (&self.command_tx, self.get_selected_entry()) {
                    let edit_time_action = EditTimeAction {
                        id: entry.id,
                        millis: entry.total_milliseconds(),
                    };
                    tx.send(Action::edit_time_action(edit_time_action))?;
                }
            }
            KeyCode::Char('d') => {
                if let (Some(tx), Some(entry)) = (&self.command_tx, self.get_selected_entry()) {
                    tx.send(Action::api_request_action(DeleteEntry { id: entry.id }))?;

                    // state will get updated after this request is processed, but
                    // removing for now so we can keep track of the selected index
                    self.entries.remove(self.selected_index);

                    // update selected index, if we're out of bounds after the deletion
                    if self.selected_index >= self.entries.len() && !self.entries.is_empty() {
                        self.selected_index = self.entries.len() - 1;
                    } else if self.entries.is_empty() {
                        self.selected_index = 0;
                    }
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
        self.rect = rect;

        let block = Block::default()
            .title(self.total_elapsed_time_string())
            .borders(Borders::ALL);
        f.render_widget(block, rect);

        let inner_area = rect.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        });

        let entry_height = 3; // Assuming each entry takes 3 lines
        let num_visible_entries = inner_area.height as usize / entry_height;

        // Create constraints for the visible entries
        let constraints: Vec<Constraint> =
            vec![Constraint::Length(entry_height as u16); num_visible_entries];
        let entry_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area);

        let start_index = self.scroll_position;
        let end_index = std::cmp::min(start_index + num_visible_entries, self.entries.len());

        for i in start_index..end_index {
            let entry_index = i % self.entries.len(); // Wrap around the index
            if let Some(chunk) = entry_chunks.get(i - start_index) {
                self.entries[entry_index].is_selected = entry_index == self.selected_index;
                self.entries[entry_index].draw(f, *chunk)?;
            }
        }

        Ok(())
    }
}
