use crate::{components::Component, tui::Frame};
use chrono::Duration;
use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::*};

use crate::api_client::models::time_entry::TimeEntryVM as ApiTimeEntry;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeEntry {
    pub charge_code: String,
    pub elapsed_time: Duration,
    pub is_active: bool,
    pub is_selected: bool,
}

impl From<&ApiTimeEntry> for TimeEntry {
    fn from(value: &ApiTimeEntry) -> Self {
        Self {
            charge_code: "TODO".to_string(),
            elapsed_time: Duration::milliseconds(value.total_time),
            is_active: value.is_active,
            is_selected: false,
        }
    }
}

impl Default for TimeEntry {
    fn default() -> Self {
        Self {
            charge_code: "Project Mgmt".to_string(),
            elapsed_time: Duration::zero(),
            is_active: false,
            is_selected: false,
        }
    }
}

impl TimeEntry {
    pub fn new() -> Self {
        Self::default()
    }

    fn format_duration(&self) -> String {
        let total_seconds = self.elapsed_time.num_seconds();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    fn get_border_style(&self) -> Style {
        if self.is_selected {
            Style::default().fg(Color::Cyan)
        } else if self.is_active {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        }
    }
}

impl Component for TimeEntry {
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let play_pause_symbol = if self.is_active { "⏸" } else { "▶" };
        let elapsed_time_str = self.format_duration();

        // Create a Block for the entire TimeEntry with the border style
        let entry_block = Block::default()
            .borders(Borders::ALL)
            .border_style(self.get_border_style());

        // Render the entire TimeEntry block
        f.render_widget(entry_block, rect);

        // Create layout within the given rect
        let inner_rect = rect.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }); // Adjust margin as needed
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(3),  // For play/pause button
                Constraint::Length(10), // For elapsed time
                Constraint::Min(10),    // For charge code
            ])
            .split(inner_rect);

        // Render play/pause button
        let button_text = Text::styled(
            play_pause_symbol,
            Style::default().add_modifier(Modifier::BOLD),
        );
        f.render_widget(Paragraph::new(button_text), chunks[0]);

        // Render elapsed time
        let time_text = Text::styled(elapsed_time_str, Style::default());
        f.render_widget(Paragraph::new(time_text), chunks[1]);

        // Render charge code
        let charge_code_text = Text::styled(&self.charge_code, Style::default());
        f.render_widget(Paragraph::new(charge_code_text), chunks[2]);

        Ok(())
    }
}
