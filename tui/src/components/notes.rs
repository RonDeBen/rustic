use super::Component;
use crate::{action::Action, tui::Frame};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tui_textarea::{Input, Key, TextArea};

#[derive(Debug, Clone, Default)]
pub struct Notes<'a> {
    pub editor: TextArea<'a>,
    pub is_edit_mode: bool,
}

impl Notes<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_text(&mut self, text: String) {
        self.editor = TextArea::new(text.lines().map(ToString::to_string).collect());
    }

    pub fn get_text(&self) -> String {
        self.editor.lines().join("\n")
    }
}

impl Component for Notes<'_> {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let title = if self.is_edit_mode {
            "TextArea (Edit Mode: 'ESC' to exit)"
        } else {
            "TextArea ('/' to enter edit mode)"
        };

        let block = Block::default().borders(Borders::ALL).title(title);

        let widget = self.editor.widget();
        f.render_widget(block, area);
        f.render_widget(widget, area);

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let tui_key: tui_textarea::Input = key.into();

        if self.is_edit_mode {
            match key.code {
                KeyCode::Esc => {
                    self.is_edit_mode = false;
                }
                _ => {
                    // Pass key events to tui-textarea
                    self.editor.input(tui_key);
                }
            }
        } else if key.code == KeyCode::Char('/') {
            self.is_edit_mode = true;
        }

        Ok(None)
    }
}
