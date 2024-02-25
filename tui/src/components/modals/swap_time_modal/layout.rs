use super::{swap_entry_list::SwapEntryList, swap_time_edit_timer::SwapTimeEdit};
use crate::{
    action::{Action, TTAct},
    api_client::ApiRequest::AddTime,
    components::{component_utils::draw_tooltip_bar, time_entry::entry::TimeEntry, Component},
    tui::Frame,
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default)]
pub struct SwapTimeModal {
    pub list_component: SwapEntryList,
    pub time_edit_component: SwapTimeEdit,
    pub is_active: bool,
    pub command_tx: Option<UnboundedSender<Action>>,
    swap_from_id: i32,
    active_area: ActiveArea,
    // other_time_entries: Vec<TimeEntry>,
}

#[derive(Default)]
enum ActiveArea {
    #[default]
    EntryList,
    TimeEdit,
}

impl SwapTimeModal {
    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;

        if self.is_active {
            self.time_edit_component.clear_time();
        }
    }

    pub fn set_swap_from_id(&mut self, id: i32) {
        self.swap_from_id = id;
    }

    pub fn set_other_entries(&mut self, other_entries: Vec<TimeEntry>) {
        self.list_component.set_time_entries(other_entries);
    }

    pub fn swap_active_area(&mut self) {
        self.active_area = match self.active_area {
            ActiveArea::EntryList => {
                self.time_edit_component.is_active = true;
                self.list_component.is_active = false;
                ActiveArea::TimeEdit
            }
            ActiveArea::TimeEdit => {
                self.list_component.is_active = true;
                self.time_edit_component.is_active = false;
                ActiveArea::EntryList
            }
        }
    }

    fn swap_time(&mut self) -> Result<Option<Action>> {
        let swap_to_entry = self.list_component.get_selected_entry();
        let swap_time = self.time_edit_component.get_total_milliseconds();

        if let (Some(swap_to_entry), Some(swap_time)) = (swap_to_entry, swap_time) {
            return self.swap_time_inner(self.swap_from_id, swap_to_entry.id, swap_time);
        }

        Ok(None)
    }

    fn swap_time_inner(
        &mut self,
        swap_from_id: i32,
        swap_to_id: i32,
        swap_time_millis: i64,
    ) -> Result<Option<Action>> {
        if let Some(tx) = &self.command_tx {
            tx.send(Action::api_request_action(AddTime {
                id: swap_from_id,
                millis: -swap_time_millis,
            }))?;

            tx.send(Action::api_request_action(AddTime {
                id: swap_to_id,
                millis: swap_time_millis,
            }))?;
        }

        self.toggle();

        Ok(None)
    }
}

impl Component for SwapTimeModal {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        if !self.is_active {
            return Ok(());
        }

        // Define margins for the modal
        let horizontal_margin = (area.width as f32 * 0.1) as u16;
        let vertical_margin = (area.height as f32 * 0.1) as u16;
        let modal_area = area.inner(&Margin {
            horizontal: horizontal_margin,
            vertical: vertical_margin,
        });

        // Clear the modal area before rendering
        f.render_widget(Clear, modal_area);

        // Calculate the height of the time edit section and tooltips
        const TIME_EDIT_HEIGHT: u16 = 6;
        const TOOLTIP_HEIGHT: u16 = 3;

        // Calculate the height for the list component
        let list_height = modal_area.height - TIME_EDIT_HEIGHT - TOOLTIP_HEIGHT;

        // Create a block for the modal
        let block = Block::default()
            .title("Swap Time")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        f.render_widget(block, modal_area);

        // Define areas for the time entry and list components
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(list_height),
                Constraint::Length(TIME_EDIT_HEIGHT),
                Constraint::Length(TOOLTIP_HEIGHT),
            ])
            .split(modal_area);

        // Draw the list component in the first chunk
        self.list_component.draw(f, chunks[0])?;

        // Draw the time edit component in the second chunk
        self.time_edit_component.draw(f, chunks[1])?;

        // Draw the tooltip bar in the third chunk
        let tooltips = vec![
            "Swap Time [Enter]",
            "Back [Esc]",
            "Switch Focus [Tab/Backtab]",
        ];
        draw_tooltip_bar(f, chunks[2], &tooltips);

        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let _ = match key.code {
            KeyCode::Enter => {
                self.swap_time()
            }
            KeyCode::Esc => {
                self.toggle();
                Ok(None)
            }
            KeyCode::Tab | KeyCode::BackTab => {
                self.swap_active_area();
                Ok(None)
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Down | KeyCode::Char('j') => {
                if matches!(self.active_area, ActiveArea::TimeEdit) {
                    self.swap_active_area()
                } else {
                    self.list_component.handle_key_events(key)?;
                }
                Ok(None)
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') => {
                if matches!(self.active_area, ActiveArea::EntryList) {
                    self.swap_active_area()
                } else {
                    self.time_edit_component.handle_key_events(key)?;
                }
                Ok(None)
            }
            _ => match self.active_area {
                ActiveArea::EntryList => self.list_component.handle_key_events(key),
                ActiveArea::TimeEdit => self.time_edit_component.handle_key_events(key),
            },
        };

        Ok(None)
    }
}
