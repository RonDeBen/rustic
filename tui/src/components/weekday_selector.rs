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

        let mut spans: Vec<Span> = Vec::new();

        for (i, day) in days.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(" | ", Style::default().dim().fg(Color::Gray)));
            }

            let day_str = format!(" {} ({}) ", day, i + 1);

            if i == self.selected_day {
                spans.push(Span::styled(
                    day_str,
                    Style::new()
                        .dim()
                        .fg(Color::White)
                        .add_modifier(Modifier::UNDERLINED),
                ));
            } else {
                spans.push(Span::styled(
                    day_str,
                    Style::default().dim().fg(Color::Gray),
                ));
            }
        }

        let line = Line::from(spans);

        let paragraph = Paragraph::new(vec![line])
            .block(
                Block::default()
                    .title("Weekday Selector")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, rect);

        Ok(())
    }
}
