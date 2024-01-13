// use super::mode_selector::ModeSelector;
use crate::{action::Action, api_client::models::charge_code::ChargeCode, components::Component};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
// use tokio::sync::mpsc::UnboundedSender;

pub struct ChargeCodePickerModal {
    pub input: String,
    pub charge_codes: Vec<ChargeCode>,
    pub filtered_codes: Vec<String>,
    pub is_active: bool,
    pub selected_charge_code_id: Option<i32>,
}

impl ChargeCodePickerModal {
    pub fn new(charge_codes: &[ChargeCode]) -> Self {
        let charge_code_names = charge_codes.iter().map(|x| x.alias.clone()).collect();
        Self {
            input: String::new(),
            charge_codes: charge_codes.to_vec(),
            filtered_codes: charge_code_names,
            is_active: false,
            selected_charge_code_id: None,
        }
    }

    pub fn set_charge_code_id(&mut self, id: i32) {
        self.selected_charge_code_id = Some(id);
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn update_input(&mut self, input: String) {
        self.input = input;
        // Here you would add your fuzzy finding logic
        // self.filtered_codes = self
        //     .charge_codes
        //     .iter()
        //     .filter(|code| code.alias.to_lowercase().contains(&self.input.to_lowercase()))
        //     .collect();
    }
    pub fn handle_char(&mut self, c: char) {
        self.input.push(c);
        self.update_input(self.input.clone());
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
            let list_items: Vec<ListItem> = self
                .filtered_codes
                .iter()
                .map(|code| ListItem::new(code.as_str()))
                .collect();
            let list_block = Block::default().borders(Borders::ALL);
            let list = List::new(list_items)
                .block(list_block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>"); // Customize the highlight symbol

            // Render the list inside the modal area, below the input
            f.render_widget(list, list_area);

            Ok(())
        } else {
            Ok(()) // If the modal is not active, do nothing
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char(c) => {
                self.handle_char(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.update_input(self.input.clone());
            }
            KeyCode::Enter => {
                // Here you would handle the selection
                // For now, let's just print the selected charge code to the console
                if let Some(selection) = self.filtered_codes.first() {
                    // TODO:
                    println!("Selected charge code: {}", selection);
                }
                self.toggle(); // Close the modal after selection
            }
            KeyCode::Esc => {
                self.toggle(); // Close the modal without selection
            }
            _ => {}
        }
        Ok(None)
    }
}
