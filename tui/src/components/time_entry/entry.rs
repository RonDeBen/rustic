use super::time_utils::format_millis;
use crate::api_client::models::time_entry::TimeEntryVM as ApiTimeEntry;
use crate::{
    action::{Action, UIAct},
    components::Component,
    tui::Frame,
};
use chrono::{Duration, NaiveDateTime, TimeZone, Utc};
use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::*};
use std::time::SystemTime;
use tokio::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeEntry {
    pub id: i32,
    pub charge_code_name: Option<String>,
    pub elapsed_time: Duration,
    pub is_active: bool,
    pub is_selected: bool,
    pub start_time: Option<Instant>,
    delta_time: Option<Duration>,
}

impl From<&ApiTimeEntry> for TimeEntry {
    fn from(value: &ApiTimeEntry) -> Self {
        let charge_code_name = value.charge_code.as_ref().map(|code| code.alias.clone());
        Self {
            id: value.id,
            charge_code_name,
            elapsed_time: Duration::milliseconds(value.total_time as i64),
            is_active: value.is_active,
            is_selected: false,
            start_time: convert_ndt_to_instant(&value.start_time),
            delta_time: None,
        }
    }
}

fn convert_ndt_to_instant(date: &Option<NaiveDateTime>) -> Option<Instant> {
    match date {
        Some(date) => {
            let datetime = Utc.from_utc_datetime(date);
            let duration_since_epoch = Duration::seconds(datetime.timestamp());

            let now_duration_since_epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            let date_duration_since_epoch = duration_since_epoch.num_seconds() as u64;

            if date_duration_since_epoch > now_duration_since_epoch {
                // The date is in the future relative to the current system time
                None
            } else {
                let elapsed = now_duration_since_epoch - date_duration_since_epoch;
                Instant::now().checked_sub(std::time::Duration::from_secs(elapsed))
            }
        }
        None => None,
    }
}

impl Default for TimeEntry {
    fn default() -> Self {
        Self {
            id: -1,
            charge_code_name: None,
            elapsed_time: Duration::zero(),
            is_active: false,
            is_selected: false,
            start_time: None,
            delta_time: None,
        }
    }
}

impl TimeEntry {
    pub fn new() -> Self {
        Self::default()
    }

    fn format_duration(&self) -> String {
        format_millis(&self.total_milliseconds())
    }

    fn get_border_style(&self) -> Style {
        match self.is_selected {
            true => Style::default().fg(Color::Yellow),
            false => Style::default().fg(Color::DarkGray),
        }
    }

    pub fn update_elapsed_time(&mut self) {
        if self.is_active {
            if let Some(start_time) = self.start_time {
                let now = Instant::now();
                let std_duration = now.duration_since(start_time);
                self.delta_time = Some(
                    Duration::seconds(std_duration.as_secs() as i64)
                        + Duration::nanoseconds(std_duration.subsec_nanos() as i64),
                );
            }
        }
    }

    pub fn total_milliseconds(&self) -> i64 {
        match self.delta_time {
            Some(d) => (self.elapsed_time + d).num_milliseconds(),
            None => self.elapsed_time.num_milliseconds(),
        }
    }

    fn charge_code_string(&self) -> &str {
        match &self.charge_code_name {
            Some(cc_name) => cc_name,
            None => "(c) to edit charge code",
        }
    }
}

impl Component for TimeEntry {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        if let Action::UI(UIAct::Tick) = action {
            self.update_elapsed_time();
        };

        Ok(None)
    }

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
                Constraint::Length(14), // For elapsed time
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
        // let charge_code_string = self.charge_code_string();
        let charge_code_text = Text::styled(self.charge_code_string(), Style::default());
        f.render_widget(Paragraph::new(charge_code_text), chunks[2]);

        Ok(())
    }
}
