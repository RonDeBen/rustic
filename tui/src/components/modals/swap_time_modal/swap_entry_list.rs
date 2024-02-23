use crate::{
    action::Action,
    components::{time_entry::entry::TimeEntry, Component},
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub struct SwapEntryList {
    pub time_entries: Vec<TimeEntry>,
    pub list_state: ListState,
    pub is_active: bool,
}

impl Default for SwapEntryList {
    fn default() -> Self {
        Self {
            time_entries: Default::default(),
            list_state: Default::default(),
            is_active: true,
        }
    }
}

impl SwapEntryList {
    pub fn set_time_entries(&mut self, entries: Vec<TimeEntry>) {
        self.time_entries = entries;
        if !self.time_entries.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn get_selected_entry(&self) -> Option<TimeEntry> {
        let selected_index = self.list_state.selected()?;
        self.time_entries.get(selected_index).cloned()
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.time_entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.time_entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn get_activity_style(&self) -> Style {
        match self.is_active {
            true => Style::default().fg(Color::Yellow),
            false => Style::default().fg(Color::DarkGray),
        }
    }
}

impl Component for SwapEntryList {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let list_block = Block::default()
            .title("Swap To Entry")
            .borders(Borders::ALL)
            .border_style(self.get_activity_style());

        // Map the TimeEntry data to ListItem
        let item_style = Style::default().fg(Color::DarkGray);
        let list_items: Vec<ListItem> = self
            .time_entries
            .iter()
            .map(|entry| {
                let label = format!(
                    "{} - {:02}:{:02}:{:02}",
                    entry
                        .charge_code_name
                        .as_ref()
                        .unwrap_or(&"No charge code".to_string()),
                    entry.elapsed_time.num_seconds() / 3600,
                    (entry.elapsed_time.num_seconds() / 60) % 60,
                    entry.elapsed_time.num_seconds() % 60,
                );
                ListItem::new(label).style(item_style)
            })
            .collect();

        let list = List::new(list_items)
            .block(list_block)
            .style(self.get_activity_style())
            .highlight_style(Style::default().fg(Color::White))
            .repeat_highlight_symbol(true)
            .highlight_symbol(">");

        f.render_stateful_widget(list, area, &mut self.list_state);

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous();
            }
            _ => {}
        }

        Ok(None)
    }
}
