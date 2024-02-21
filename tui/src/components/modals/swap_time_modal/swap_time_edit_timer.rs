use crate::{action::Action, components::Component};
use color_eyre::eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, ListState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

pub struct SwapTimeEdit {
    time_string: String,
    cursor_pos: u8,
    pub is_active: bool,
    pub entry_id: i32,
    command_tx: Option<UnboundedSender<Action>>,
}

impl Default for SwapTimeEdit {
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

impl Component for SwapTimeEdit {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let block = Block::default()
            .title("Time Edit")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        f.render_widget(block, area);
        Ok(())
    }
}
