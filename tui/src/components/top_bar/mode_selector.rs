use crate::{action::Action, components::Component, mode::Mode, tui::Frame};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use crate::action::TTAct::UpdateMode;
#[derive(Debug, Clone, Default)]
pub struct ModeSelector {
    selected_mode: Mode,
    command_tx: Option<UnboundedSender<Action>>,
}

impl ModeSelector {
    pub fn new() -> Self {
        Self::default()
    }

    fn select_mode(&mut self, mode: Mode) {
        self.selected_mode = mode;
        if let Some(tx) = &self.command_tx {
            tx.send(Action::TT(UpdateMode(mode)))
                .unwrap();
        }
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

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());

        Ok(())
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
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Right)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, rect);

        Ok(())
    }
}
