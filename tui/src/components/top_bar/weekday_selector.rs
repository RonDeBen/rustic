use crate::{
    action::{Action, TTAct::ChangeDay},
    api_client::models::day::Day,
    components::Component,
    tui::Frame,
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone, Default)]
pub struct WeekdaySelector {
    selected_day: Day,
    command_tx: Option<UnboundedSender<Action>>,
}

impl WeekdaySelector {
    pub fn new() -> Self {
        Self::default()
    }

    fn select_day(&mut self, day: Day) -> Result<()> {
        self.selected_day = day;
        if let Some(tx) = &self.command_tx {
            tx.send(Action::TT(ChangeDay(day)))?;
        }
        Ok(())
    }
}

impl Component for WeekdaySelector {
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];

        let mut spans: Vec<Span> = Vec::new();

        for (i, day) in days.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(" | ", Style::default().dim().fg(Color::Gray)));
            }

            let day_str = format!(" {} ({}) ", day, i + 1);

            if Day::from(i as i16) == self.selected_day {
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

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('1') => self.select_day(Day::Monday)?,
            KeyCode::Char('2') => self.select_day(Day::Tuesday)?,
            KeyCode::Char('3') => self.select_day(Day::Wednesday)?,
            KeyCode::Char('4') => self.select_day(Day::Thursday)?,
            KeyCode::Char('5') => self.select_day(Day::Friday)?,
            _ => {}
        };
        Ok(None)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());

        Ok(())
    }
}
