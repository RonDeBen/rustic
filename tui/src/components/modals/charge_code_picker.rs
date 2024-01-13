// use super::mode_selector::ModeSelector;
use crate::{action::Action, api_client::models::charge_code::ChargeCode, components::Component};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::{
    layout::{Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub struct ChargeCodePickerModal {
    pub input: String,
    pub charge_codes: Vec<ChargeCode>,
    pub filtered_codes: Vec<String>,
    pub is_active: bool,
    pub selected_charge_code_id: Option<i32>,
    pub list_state: ListState,
}

impl ChargeCodePickerModal {
    pub fn new(charge_codes: &[ChargeCode]) -> Self {
        let charge_code_names = charge_codes.iter().map(|x| x.alias.clone()).collect();
        let mut list_state = ListState::default();
        if !charge_codes.is_empty(){
            list_state.select(Some(0))
        }
        Self {
            input: String::new(),
            charge_codes: charge_codes.to_vec(),
            filtered_codes: charge_code_names,
            is_active: false,
            selected_charge_code_id: None,
            list_state
        }
    }

    pub fn set_charge_code_id(&mut self, id: i32) {
        self.selected_charge_code_id = Some(id);
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn update_selection_to_first(&mut self) {
        if !self.filtered_codes.is_empty(){
            self.list_state.select(Some(0));
        }
    }

    pub fn update_input(&mut self, input: String) {
        self.input = input;
        self.filtered_codes = self
            .charge_codes
            .iter()
            .filter(|code| {
                code.alias
                    .to_lowercase()
                    .contains(&self.input.to_lowercase())
            })
            .map(|code| code.alias.clone())
            .collect();
        self.update_selection_to_first()
    }

    pub fn handle_char(&mut self, c: char) {
        self.input.push(c);
        self.update_input(self.input.clone());
    }

    fn delete_previous_word(&mut self) {
        if let Some(pos) = self.input.rfind(' ') {
            self.input.truncate(pos);
        } else {
            self.input.clear();
        }
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_codes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_codes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

impl Component for ChargeCodePickerModal {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        if self.is_active {
            // Define the size and position of the modal
            let modal_area = area.inner(&Margin {
                horizontal: area.width / 4,
                vertical: area.height / 4,
            });

            // Clear the area before rendering the modal to ensure it's on top
            f.render_widget(Clear, modal_area);

            // Create a block for the modal
            let block = Block::default()
                .title("Charge Code Selection")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow));

            // Render the block around the modal area
            f.render_widget(block, modal_area);

            // Draw the input box
            let input_block = Block::default().title("Input").borders(Borders::ALL);
            let input_paragraph = Paragraph::new(&*self.input)
                .block(input_block)
                .wrap(Wrap { trim: true });
            f.render_widget(input_paragraph, modal_area);

            // Calculate the area below the input box to list the filtered charge codes
            let list_area = Rect {
                x: modal_area.x,
                y: modal_area.y + 3, // Just below the input box
                width: modal_area.width,
                height: modal_area.height - 3, // Remaining height
            };

            // Create and draw the list of charge codes
            let list_block = Block::default().borders(Borders::ALL);
            let item_style = Style::default().fg(Color::DarkGray);
            let list_items: Vec<ListItem> = self
                .filtered_codes
                .iter()
                .map(|code| ListItem::new(code.clone()).style(item_style))
                .collect();


            let list = List::new(list_items)
                .block(list_block)
                .style(Style::default().fg(Color::Yellow))
                .highlight_style(Style::default().fg(Color::White))
                .repeat_highlight_symbol(true)
                .highlight_symbol(">");

            f.render_stateful_widget(list, list_area, &mut self.list_state);

            Ok(())
        } else {
            Ok(()) // If the modal is not active, do nothing
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key {
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => {
                self.delete_previous_word(); // A method to delete the previous word
                self.update_input(self.input.clone());
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => {
                self.previous();
            }
            _ => match key.code {
                KeyCode::Char(c) => {
                    self.handle_char(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                    self.update_input(self.input.clone());
                }
                KeyCode::Enter => {
                    if let Some(selection) = self.filtered_codes.first() {
                        // TODO: handle selection
                        println!("Selected charge code: {}", selection);
                    }
                    self.toggle();
                }
                KeyCode::Esc => {
                    self.toggle();
                }
                KeyCode::Tab | KeyCode::Down => {
                    self.next();
                }
                KeyCode::Up => {
                    self.previous();
                }
                _ => {}
            },
        }
        Ok(None)
    }
}
