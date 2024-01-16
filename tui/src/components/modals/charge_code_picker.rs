use crate::components::component_utils::draw_tooltip_bar;
use crate::components::Component;
use crate::{action::Action, api_client::ApiRequest::UpdateChargeCode};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::layout::Alignment;
use ratatui::{
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use shared_lib::models::charge_code::ChargeCode;
use tokio::sync::mpsc::UnboundedSender;

pub struct ChargeCodePickerModal {
    command_tx: Option<UnboundedSender<Action>>,
    matcher: SkimMatcherV2,
    pub input: String,
    pub charge_codes: Vec<ChargeCode>,
    pub filtered_codes: Vec<ChargeCodeRef>,
    pub is_active: bool,
    pub entry_id: Option<i32>,
    pub list_state: ListState,
}

pub struct ChargeCodeRef {
    alias: String,
    id: i32,
}

impl ChargeCodePickerModal {
    pub fn new(charge_codes: &[ChargeCode]) -> Self {
        let charge_code_names = charge_codes
            .iter()
            .map(|x| ChargeCodeRef {
                alias: x.alias.clone(),
                id: x.id,
            })
            .collect();
        let mut list_state = ListState::default();
        if !charge_codes.is_empty() {
            list_state.select(Some(0))
        }
        Self {
            command_tx: None,
            input: String::new(),
            charge_codes: charge_codes.to_vec(),
            filtered_codes: charge_code_names,
            is_active: false,
            entry_id: None,
            list_state,
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn set_charge_code_id(&mut self, id: i32) {
        self.entry_id = Some(id);
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
        self.update_input("".to_string());
        self.update_selection_to_first();
    }

    fn update_selection_to_first(&mut self) {
        if !self.filtered_codes.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn update_input(&mut self, input: String) {
        self.input = input;

        let mut scored_codes: Vec<(i64, String, i32)> = self
            .charge_codes
            .iter()
            .filter_map(|code| {
                self.matcher
                    .fuzzy_match(&code.alias, &self.input)
                    .map(|score| (score, code.alias.clone(), code.id))
            })
            .collect();

        scored_codes.sort_by(|a, b| b.0.cmp(&a.0));

        self.filtered_codes = scored_codes
            .into_iter()
            .map(|(_, alias, id)| ChargeCodeRef { alias, id })
            .collect();

        self.update_selection_to_first();
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

    fn get_selected_charge_code_id(&self) -> Option<i32> {
        self.list_state.selected().and_then(|selected_index| {
            self.filtered_codes
                .get(selected_index)
                .map(|code_ref| code_ref.id)
        })
    }
}

impl Component for ChargeCodePickerModal {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        if !self.is_active {
            Ok(())
        } else {
            let horizontal_margin = (area.width as f32 * 0.1) as u16;
            let vertical_margin = (area.height as f32 * 0.1) as u16;

            // Define the size and position of the modal
            let modal_area = area.inner(&Margin {
                horizontal: horizontal_margin,
                vertical: vertical_margin,
            });

            // Clear the area before rendering the modal to ensure it's on top
            f.render_widget(Clear, modal_area);

            // Define the area for the bottom bar within the modal
            let bottom_bar_area = Rect {
                x: modal_area.x,
                y: modal_area.y + modal_area.height - 3, // Position the bottom bar 3 rows from the bottom
                width: modal_area.width,
                height: 3, // Height of the bottom bar (3 rows)
            };

            // Create a block for the modal
            let block = Block::default()
                .title("Charge Code Selection")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow));

            // Render the block around the modal area
            f.render_widget(block, modal_area);

            // Draw the input box
            let input_block = Block::default()
                .title(" Find Charge Code ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);
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
                .map(|code_ref| ListItem::new(code_ref.alias.clone()).style(item_style))
                .collect();

            let list = List::new(list_items)
                .block(list_block)
                .style(Style::default().fg(Color::Yellow))
                .highlight_style(Style::default().fg(Color::White))
                .repeat_highlight_symbol(true)
                .highlight_symbol(">");

            f.render_stateful_widget(list, list_area, &mut self.list_state);

            let tooltips = vec!["Select [Enter]", "Back [Esc]"];
            draw_tooltip_bar(f, bottom_bar_area, &tooltips);

            Ok(())
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('w') {
            self.delete_previous_word();
            self.update_input(self.input.clone());
        } else if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::BackTab {
            self.previous();
        } else {
            match key.code {
                KeyCode::Char(c) => {
                    self.handle_char(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                    self.update_input(self.input.clone());
                }
                KeyCode::Enter => {
                    if let (Some(time_entry_id), Some(charge_code_id), Some(tx)) = (
                        self.entry_id,
                        self.get_selected_charge_code_id(),
                        &self.command_tx,
                    ) {
                        tx.send(Action::api_request_action(UpdateChargeCode {
                            time_entry_id,
                            charge_code_id,
                        }))?;
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
            }
        }

        Ok(None)
    }
}
