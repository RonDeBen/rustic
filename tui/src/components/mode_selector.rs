use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use super::Component;
use crate::{action::Action, mode::Mode, tui::Frame};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModeSelector {
    selected_mode: Mode,
}

impl ModeSelector {
    pub fn new() -> Self {
        Self::default()
    }

    fn select_mode(&mut self, mode: Mode) {
        self.selected_mode = mode;
    }

    fn style_by_is_mode_selected(&mut self, mode: Mode) -> Style {
        match self.selected_mode == mode {
            true => Style::default()
                .dim()
                .fg(Color::White)
                .add_modifier(Modifier::UNDERLINED),
            false => Style::default().dim().fg(Color::Gray),
        }
    }
}

impl Component for ModeSelector {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('9') => self.select_mode(Mode::Crud),
            KeyCode::Char('0') => self.select_mode(Mode::Standup),
            _ => {}
        };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let line = Line::from(vec![
            Span::styled("CRUD (9)", self.style_by_is_mode_selected(Mode::Crud)),
            Span::styled(" | ", Style::default().dim().fg(Color::Gray)),
            Span::styled("Standup (0)", self.style_by_is_mode_selected(Mode::Standup)),
        ]);

        let paragraph = Paragraph::new(vec![line])
            .block(
                Block::default()
                    .title("Mode Selector")
                    .title_alignment(Alignment::Right)
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Right)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, rect);

        Ok(())
    }
}
