use crate::api_client::ApiRequest::UpdateEntryNote;
use crate::components::Component;
use crate::{action::Action, tui::Frame};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{block::Block, Borders};
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

use super::vim::{Mode, Transition, Vim};

#[derive(Debug, Clone, Default)]
pub struct VimnNotes<'a> {
    pub editor: TextArea<'a>,
    pub vim: Vim,
    pub is_edit_mode: bool,
    entry_id: i32,
    command_tx: Option<UnboundedSender<Action>>,
}

impl VimnNotes<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_text(&mut self, text: String) {
        self.editor = TextArea::new(text.lines().map(ToString::to_string).collect());
    }

    pub fn set_id(&mut self, id: i32) {
        self.entry_id = id;
    }

    pub fn get_text(&self) -> String {
        self.editor.lines().join("\n")
    }
}

impl Component for VimnNotes<'_> {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        // Determine title and border style based on whether editing is enabled
        let title = if self.is_edit_mode {
            match self.vim.mode {
                Mode::Normal => format!("{} - 'i' to insert, 'v' to visual, Esc to quit", Mode::Normal),
                Mode::Insert => format!("{} - Esc to normal", Mode::Insert),
                Mode::Visual => format!("{} - 'y' to copy, 'd' to cut, Esc to normal", Mode::Visual),
                Mode::Operator(_) => format!("{}", self.vim.mode),
            }
        } else {
            "Press '/' to start editing".to_string()
        };

        let border_color = match self.is_edit_mode {
            true => match self.vim.mode {
                Mode::Normal => Color::LightGreen,
                Mode::Insert => Color::LightBlue,
                Mode::Visual => Color::LightYellow,
                Mode::Operator(_) => Color::LightMagenta,
            },
            false => Color::Gray,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(border_color));

        // Set cursor style based on mode, if editing
        if self.is_edit_mode {
            let cursor_style = self.vim.mode.cursor_style();
            self.editor.set_cursor_style(cursor_style);
        }

        // Apply block to editor and render
        self.editor.set_block(block);
        f.render_widget(self.editor.widget(), area);

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if self.is_edit_mode {
            let input: tui_textarea::Input = key.into(); // Convert KeyEvent to Input for the TextArea

            match self.vim.transition(input, &mut self.editor) {
                Transition::Mode(mode) => {
                    self.vim = Vim::new(mode);
                }
                Transition::Quit => {
                    if let Some(tx) = &self.command_tx {
                        let update_note_action = UpdateEntryNote {
                            id: self.entry_id,
                            note: self.get_text(),
                        };
                        tx.send(Action::api_request_action(update_note_action))?;
                    }
                    self.is_edit_mode = false;
                }
                _ => {} // Handle other transitions or inputs
            }
        } else if key.code == KeyCode::Char('/') {
            self.is_edit_mode = true;
            self.vim = Vim::new(Mode::Insert);
        }

        Ok(None)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);

        Ok(())
    }
}
