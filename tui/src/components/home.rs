use super::{
    component_utils::draw_tooltip_bar,
    modals::{charge_code_picker::ChargeCodePickerModal, time_edit_modal::TimeEditModal},
    notes::Notes,
    time_entry::time_entry_container::TimeEntryContainer,
    top_bar::layout::TopBar,
    Component, Frame,
};
use crate::{
    action::{
        Action, ApiAct, TTAct,
        UIAct::{self, *},
    },
    api_client::{
        models::{day::Day, FullState},
        ApiResponse,
    },
    config::Config,
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

pub struct Home<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    // components
    top_bar: TopBar,
    time_entry_container: TimeEntryContainer,
    notes: Notes<'a>,
    charge_code_modal: ChargeCodePickerModal,
    time_edit_modal: TimeEditModal,
    // data
    full_state: FullState,
    current_day: Day,
}

impl Home<'_> {
    pub fn new(starting_state: FullState) -> Self {
        let current_day = Day::get_current_day();
        let current_entries = starting_state.get_time_entries_for_day(current_day);
        let time_entry_container = TimeEntryContainer::new(current_entries, 0);
        let charge_code_modal = ChargeCodePickerModal::new(starting_state.charge_codes.as_slice());
        Self {
            command_tx: None,
            config: Config::default(),
            top_bar: TopBar::new(current_day),
            time_entry_container,
            charge_code_modal,
            time_edit_modal: TimeEditModal::default(),
            notes: Notes::default(),
            full_state: starting_state,
            current_day,
        }
    }

    fn set_time_entries(&mut self) {
        self.time_entry_container
            .set_time_entries(self.full_state.get_time_entries_for_day(self.current_day));

        // this will reset the note whenever time entries get set (all the time)
        // this also handles updating the notes when switching days
        let id = match self.time_entry_container.get_selected_entry() {
            Some(entry) => entry.id,
            None => 0,
        };
        self.set_note_for_id(id);
    }

    fn set_note_for_id(&mut self, note_id: i32) {
        self.notes.set_id(note_id);
        let text = self
            .full_state
            .get_vms_for_day(self.current_day)
            .and_then(|entries| entries.iter().find(|e| e.id == note_id))
            .map_or("".to_string(), |entry| entry.note.clone());

        self.notes.set_text(text);
    }

    fn handle_response(&mut self, respo: ApiResponse) {
        match respo {
            ApiResponse::FullState(state) => self.full_state = state,
            ApiResponse::DayEntriesUpdate(day_entries) => {
                self.full_state
                    .time_entries
                    .insert(day_entries.day, day_entries.entries);
            }
            ApiResponse::TimeEntryUpdate(entry) => {
                if let Some(entries) = self.full_state.time_entries.get_mut(&entry.day) {
                    for existing_entry in entries.iter_mut() {
                        if existing_entry.id == entry.id {
                            *existing_entry = entry.clone();
                            break;
                        }
                    }
                }
            }
        }
    }
}

impl Component for Home<'_> {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());
        self.top_bar.register_action_handler(tx.clone())?;
        self.time_entry_container
            .register_action_handler(tx.clone())?;
        self.notes.register_action_handler(tx.clone())?;
        self.charge_code_modal.register_action_handler(tx.clone())?;
        self.time_edit_modal.register_action_handler(tx.clone())?;

        // will initialize the note to the right entry id at start
        self.time_entry_container.reset_index();

        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::UI(ui_action) => {
                if ui_action == UIAct::Tick {
                    self.time_entry_container.update(Action::UI(ui_action))?;
                }
            }
            Action::TT(tt_action) => match tt_action {
                TTAct::ChangeDay(day) => {
                    self.current_day = day;
                    self.set_time_entries();
                    self.time_entry_container.reset_index();
                }
                TTAct::UpdateNote(note_id) => {
                    self.set_note_for_id(note_id);
                }
                TTAct::EditChargeCode(id) => {
                    self.charge_code_modal.set_charge_code_id(id);
                    self.charge_code_modal.toggle();
                }
                TTAct::EditTime(time_action) => {
                    self.time_edit_modal.set_time(time_action.millis);
                    self.time_edit_modal.set_entry_id(time_action.id);
                    self.time_edit_modal.toggle();
                }
            },
            Action::Api(api_action) => {
                // only handle the responses here
                if let ApiAct::Response(respo) = api_action {
                    self.handle_response(respo);
                    self.set_time_entries();
                }
            }
        }
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if self.notes.is_edit_mode {
            self.notes.handle_key_events(key)?;
        } else if self.charge_code_modal.is_active {
            self.charge_code_modal.handle_key_events(key)?;
        } else if self.time_edit_modal.is_active {
            self.time_edit_modal.handle_key_events(key)?;
        } else {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(Action::UI(Quit))),
                _ => {
                    self.top_bar.handle_key_events(key)?;
                    self.time_entry_container.handle_key_events(key)?;
                    self.notes.handle_key_events(key)?;
                }
            }
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let top_bar_height = 3;
        let bottom_bar_height = 3;

        let remaining_space_height = area.height - top_bar_height - bottom_bar_height;

        let time_entry_container_height = (remaining_space_height as f32 * 0.75) as u16;
        let notes_height = remaining_space_height - time_entry_container_height;

        let layout = Layout::new()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(top_bar_height), // Fixed height for top bar
                Constraint::Length(time_entry_container_height), // 75% of remaining space for time_entry_container
                Constraint::Length(notes_height),                // Remaining space for notes
                Constraint::Length(bottom_bar_height),           // Fixed height for bottom bar
            ])
            .split(area);

        self.top_bar.draw(f, layout[0])?;
        self.time_entry_container.draw(f, layout[1])?;

        // Draw the modals over the time entry container if they are active
        if self.charge_code_modal.is_active {
            self.charge_code_modal.draw(f, layout[1])?;
        }
        if self.time_edit_modal.is_active {
            self.time_edit_modal.draw(f, layout[1])?;
        }

        self.notes.draw(f, layout[2])?;

        let tooltips = vec![
            "Quit [q]",
            "Add [a]",
            "Delete [d]",
            "Play/Pause [Space]",
            "Code [c]",
            "Time [t]",
        ];
        draw_tooltip_bar(f, layout[3], &tooltips);

        Ok(())
    }
}
