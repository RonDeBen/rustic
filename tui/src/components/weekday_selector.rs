use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use super::Component;
use crate::{action::Action, tui::Frame};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WeekdaySelector {
    selected_day: usize, // 0 for Mon, 1 for Tue, ..., 4 for Fri
}

impl WeekdaySelector {
    pub fn new() -> Self {
        Self::default()
    }

    fn select_day(&mut self, day: usize) {
        if day < 5 {
            self.selected_day = day;
        }
    }
}

impl Component for WeekdaySelector {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('1') => self.select_day(0),
            KeyCode::Char('2') => self.select_day(1),
            KeyCode::Char('3') => self.select_day(2),
            KeyCode::Char('4') => self.select_day(3),
            KeyCode::Char('5') => self.select_day(4),
            _ => {}
        };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];

        // Create a vector of Spans, each representing a day of the week
        let spans: Vec<Span> = days
            .iter()
            .enumerate()
            .map(|(i, day)| {
                let day_str = format!(" [{} ({})] ", day, i + 1);
                if i == self.selected_day {
                    // If it's the selected day, underline it
                    Span::styled(day_str, Style::default().add_modifier(Modifier::UNDERLINED))
                } else {
                    // Otherwise, just display it normally
                    Span::raw(day_str)
                }
            })
            .collect();

        // Create a Line from the spans
        let line = Line::from(spans);

        // Create a Paragraph with the line
        let paragraph = Paragraph::new(vec![line])
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        // Render the paragraph in the specified area
        f.render_widget(paragraph, rect);

        Ok(())
    }
}
