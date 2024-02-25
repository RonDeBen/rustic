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

#[derive(Debug, Clone, Default)]
pub struct VimnNotes<'a> {
    pub editor: TextArea<'a>,
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
        let title = if self.is_edit_mode {
            "'ESC' to stop editing"
        } else {
            "'/' to edit"
        };

        self.editor.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(match self.is_edit_mode {
                    true => Style::default().fg(Color::Yellow),
                    false => Style::default().fg(Color::White),
                }),
        );

        f.render_widget(self.editor.widget(), area);

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let tui_key: tui_textarea::Input = key.into();

        if self.is_edit_mode {
            match key.code {
                KeyCode::Esc => {
                    if let Some(tx) = &self.command_tx {
                        let update_note_action = UpdateEntryNote {
                            id: self.entry_id,
                            note: self.get_text(),
                        };
                        tx.send(Action::api_request_action(update_note_action))?;
                    }
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

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);

        Ok(())
    }
}
