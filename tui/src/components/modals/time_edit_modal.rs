use crate::components::Component;
use crate::components::component_utils::draw_tooltip_bar;
use crate::{action::Action, api_client::ApiRequest::SetTime};
use chrono::offset;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Stylize};
use ratatui::text::{Line, Span};
use ratatui::{
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

pub struct TimeEditModal {
    time_string: String,
    cursor_pos: u8,
    pub is_active: bool,
    pub entry_id: i32,
    command_tx: Option<UnboundedSender<Action>>,
}

impl Default for TimeEditModal {
    fn default() -> Self {
        Self {
            time_string: "000000".to_string(),
            cursor_pos: 1,
            is_active: false,
            entry_id: -1,
            command_tx: None,
        }
    }
}

enum TimeUnit {
    Hours,
    Minutes,
    Seconds,
}

impl TimeEditModal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_time(&mut self, milliseconds: i64) {
        let total_seconds = milliseconds / 1000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        self.time_string = format!("{:02}{:02}{:02}", hours, minutes, seconds);
    }

    fn get_total_milliseconds(&self) -> Option<i64> {
        let hours = self.time_string[0..2].parse::<i64>().ok()?;
        let minutes = self.time_string[2..4].parse::<i64>().ok()?;
        let seconds = self.time_string[4..6].parse::<i64>().ok()?;

        let total_milliseconds = (hours * 3600 + minutes * 60 + seconds) * 1000;
        Some(total_milliseconds)
    }

    pub fn set_entry_id(&mut self, id: i32) {
        self.entry_id = id;
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
        self.cursor_pos = 1;
    }

    fn handle_digit_input(&mut self, c: char) {
        if self.is_valid_input(c) {
            let idx = self.cursor_pos as usize;
            self.time_string.replace_range(idx..idx + 1, &c.to_string());
            self.move_cursor_right();
        }
    }

    fn is_valid_input(&self, c: char) -> bool {
        match self.cursor_pos {
            0 => c <= '2',
            1 => {
                (c <= '3' && self.time_string.starts_with('2'))
                    || (c <= '9' && !self.time_string.starts_with('2'))
            }
            2 | 4 => c <= '5',
            3 | 5 => c.is_ascii_digit(),
            _ => false,
        }
    }

    fn move_cursor_right(&mut self) {
        self.cursor_pos = (self.cursor_pos + 1) % 6;
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos == 0 {
            self.cursor_pos = 5;
        } else {
            self.cursor_pos -= 1;
        }
    }

    fn move_focus_forward(&mut self) {
        self.cursor_pos = (self.cursor_pos / 2 * 2 + 2) % 6;
    }

    fn move_focus_backward(&mut self) {
        self.cursor_pos = if self.cursor_pos == 0 || self.cursor_pos == 1 {
            4
        } else {
            self.cursor_pos / 2 * 2 - 2
        };
    }

    fn get_hours(&self) -> String {
        self.time_string[0..2].to_string()
    }

    fn get_minutes(&self) -> String {
        self.time_string[2..4].to_string()
    }

    fn get_seconds(&self) -> String {
        self.time_string[4..6].to_string()
    }

    fn get_value_from_unit(&self, unit: &TimeUnit) -> String {
        match unit {
            TimeUnit::Hours => self.get_hours(),
            TimeUnit::Minutes => self.get_minutes(),
            TimeUnit::Seconds => self.get_seconds(),
        }
    }

    fn get_title_from_unit(&self, unit: &TimeUnit) -> &str {
        match unit {
            TimeUnit::Hours => "Hours",
            TimeUnit::Minutes => "Minutes",
            TimeUnit::Seconds => "Seconds,",
        }
    }

    fn is_component_active(&self, unit: &TimeUnit) -> bool {
        match unit {
            TimeUnit::Hours => self.cursor_pos == 0 || self.cursor_pos == 1,
            TimeUnit::Minutes => self.cursor_pos == 2 || self.cursor_pos == 3,
            TimeUnit::Seconds => self.cursor_pos == 4 || self.cursor_pos == 5,
        }
    }

    fn is_active_digit(&self, index: usize, unit: &TimeUnit) -> bool {
        let offset = match unit {
            TimeUnit::Hours => 0,
            TimeUnit::Minutes => 2,
            TimeUnit::Seconds => 4,
        };

        (index + offset) == self.cursor_pos as usize
    }

    fn draw_time_component(&self, f: &mut Frame<'_>, area: Rect, unit: TimeUnit) {
        let value = self.get_value_from_unit(&unit);
        let title = self.get_title_from_unit(&unit);

        let mut spans = Vec::new();
        for (i, digit) in value.chars().enumerate() {
            let style = match self.is_active_digit(i, &unit) {
                true => Style::default().dim().fg(Color::Black).bg(Color::Cyan),
                false => Style::default().dim().fg(Color::White),
            };

            // TODO: use this crate: https://docs.rs/tui-big-text/0.3.2/tui_big_text/
            // to try and make the numbers bigger
            spans.push(Span::styled(digit.to_string(), style));
        }

        let border_style = match self.is_component_active(&unit) {
            true => Style::default().fg(Color::Cyan),
            false => Style::default().fg(Color::White),
        };

        let paragraph = Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(border_style),
            );

        f.render_widget(paragraph, area);
    }
}

impl Component for TimeEditModal {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        if !self.is_active {
            Ok(())
        } else {

            let horizontal_margin = (area.width as f32 * 0.2) as u16;
            let vertical_margin = (area.height as f32 * 0.2) as u16;

            // Define the size and position of the modal
            let modal_area = area.inner(&Margin {
                horizontal: horizontal_margin,
                vertical: vertical_margin,
            });

            // Clear the area before rendering the modal to ensure it's on top
            f.render_widget(Clear, modal_area);

            // Create a block for the modal
            let block = Block::default()
                .title("Edit Time")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow));
            f.render_widget(block, modal_area);

            // Define the area for the bottom bar within the modal
            let bottom_bar_area = Rect {
                x: modal_area.x,
                y: modal_area.y + modal_area.height - 3, // Position the bottom bar 3 rows from the bottom
                width: modal_area.width,
                height: 3, // Height of the bottom bar (3 rows)
            };

            // Draw the tooltips in the bottom bar
            let tooltips = vec!["Set Time [Enter]", "Back [Esc]"];
            draw_tooltip_bar(f, bottom_bar_area, &tooltips);

            let horizontal_margin = (modal_area.width as f32 * 0.1) as u16;
            let vertical_margin = (modal_area.height as f32 * 0.2) as u16;

            let timer_area = modal_area.inner(&Margin {
                horizontal: horizontal_margin,
                vertical: vertical_margin,
            });

            // Define areas for hours, minutes, and seconds
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(timer_area);

            // Draw each time component
            self.draw_time_component(f, chunks[0], TimeUnit::Hours);
            self.draw_time_component(f, chunks[1], TimeUnit::Minutes);
            self.draw_time_component(f, chunks[2], TimeUnit::Seconds);

            Ok(())
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if self.is_active {
            match key.code {
                KeyCode::Esc => {
                    self.toggle();
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    self.handle_digit_input(c);
                }
                KeyCode::Tab => {
                    self.move_focus_forward();
                }
                KeyCode::BackTab => {
                    self.move_focus_backward();
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.move_cursor_left();
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.move_cursor_right();
                }
                KeyCode::Enter => {
                    if let (Some(tx), Some(millis)) =
                        (&self.command_tx, self.get_total_milliseconds())
                    {
                        tx.send(Action::api_request_action(SetTime {
                            id: self.entry_id,
                            millis,
                        }))?;
                    }

                    self.toggle();
                }
                _ => {}
            }
        }
        Ok(None)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());

        Ok(())
    }
}

