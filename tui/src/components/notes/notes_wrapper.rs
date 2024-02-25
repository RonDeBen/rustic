use super::{regular_notes::RegularNotes, vim_notes::VimnNotes};
use crate::{action::Action, components::Component, config::Config, tui::Frame};
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

pub struct NotesWrapper<'a> {
    regular_notes: RegularNotes<'a>,
    vim_notes: VimnNotes<'a>,
    use_vim_mode: bool,
}

impl NotesWrapper<'_> {
    pub fn set_text(&mut self, text: String) {
        match self.use_vim_mode {
            true => self.vim_notes.set_text(text),
            false => self.regular_notes.set_text(text),
        }
    }

    pub fn set_id(&mut self, id: i32) {
        match self.use_vim_mode {
            true => self.vim_notes.set_id(id),
            false => self.regular_notes.set_id(id),
        }
    }

    pub fn get_text(&self) -> String {
        match self.use_vim_mode {
            true => self.vim_notes.get_text(),
            false => self.regular_notes.get_text(),
        }
    }

    pub fn is_edit_mode(&self) -> bool {
        match self.use_vim_mode {
            true => self.vim_notes.is_edit_mode,
            false => self.regular_notes.is_edit_mode,
        }
    }
}

impl<'a> NotesWrapper<'a> {
    pub fn new(config: &Config) -> Self {
        Self {
            regular_notes: RegularNotes::new(),
            vim_notes: VimnNotes::new(),
            use_vim_mode: config.vim_mode.enabled,
        }
    }

    pub fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        if self.use_vim_mode {
            self.vim_notes.draw(f, area)
        } else {
            self.regular_notes.draw(f, area)
        }
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if self.use_vim_mode {
            self.vim_notes.handle_key_events(key)
        } else {
            self.regular_notes.handle_key_events(key)
        }
    }

    pub fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        match self.use_vim_mode {
            true => self.vim_notes.register_action_handler(tx),
            false => self.regular_notes.register_action_handler(tx),
        }?;

        Ok(())
    }
}
