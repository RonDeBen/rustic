use super::{mode_selector::ModeSelector, weekday_selector::WeekdaySelector};
use crate::{action::Action, components::Component};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use shared_lib::models::day::Day;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone, Default)]
pub struct TopBar {
    weekday_selector: WeekdaySelector,
    mode_selector: ModeSelector,
}

impl TopBar {
    fn is_mode_selector_event(&self, key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Char('0') | KeyCode::Char('9'))
    }

    pub fn new(selected_day: Day) -> Self {
        Self {
            weekday_selector: WeekdaySelector::new(selected_day),
            mode_selector: ModeSelector::default(),
        }
    }
}

impl Component for TopBar {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let layout = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Percentage(75), Constraint::Percentage(25)],
        )
        .split(area);

        self.weekday_selector.draw(f, layout[0])?;

        self.mode_selector.draw(f, layout[1])?;

        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.weekday_selector.register_action_handler(tx.clone())?;
        self.mode_selector.register_action_handler(tx)?;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.weekday_selector.handle_key_events(key)?;

        if self.is_mode_selector_event(&key) {
            self.mode_selector.handle_key_events(key)?;
        }

        Ok(None)
    }
}
