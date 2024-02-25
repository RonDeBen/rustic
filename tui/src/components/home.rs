use super::{
    component_utils::draw_tooltip_bar,
    modals::{
        charge_code_picker::ChargeCodePickerModal, swap_time_modal::layout::SwapTimeModal,
        time_edit_modal::TimeEditModal,
    },
    notes::Notes,
    standup::standup_container::StandupContainer,
    time_entry::{entry::TimeEntry, time_entry_container::TimeEntryContainer},
    top_bar::layout::TopBar,
    Component, Frame,
};
use crate::{
    action::{
        Action, ApiAct, TTAct,
        UIAct::{self, *},
    },
    action_history::ActionHistory,
    api_client::{models::FullStateExt, ApiRequest, ApiResponse},
    config::Config,
    mode::Mode,
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;
use shared_lib::models::{
    day::Day,
    full_state::{FullState, TimeEntriesDiff},
};
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
    swap_time_modal: SwapTimeModal,
    standup_container: StandupContainer,
    // data
    full_state: FullState,
    current_day: Day,
    mode: Mode,
    // handles undos and redos
    state_history: ActionHistory,
}

impl Home<'_> {
    pub fn new(starting_state: FullState) -> Self {
        let current_day = Day::get_current_day();
        let current_entries = starting_state.get_current_time_entries();
        let time_entry_container = TimeEntryContainer::new(current_entries, 0, current_day);
        let charge_code_modal = ChargeCodePickerModal::new(starting_state.charge_codes.as_slice());

        Self {
            command_tx: None,
            config: Config::default(),
            top_bar: TopBar::new(current_day),
            time_entry_container,
            charge_code_modal,
            time_edit_modal: TimeEditModal::default(),
            swap_time_modal: SwapTimeModal::default(),
            notes: Notes::default(),
            full_state: starting_state,
            current_day,
            mode: Mode::default(),
            standup_container: StandupContainer::default(),
            state_history: ActionHistory::default(),
        }
    }

    fn undo_action(&mut self) {
        if let Some(previous_state) = self.state_history.undo() {
            self.state_history.redo_stack.push(self.full_state.clone());
            self.apply_diff(previous_state);
        }
    }

    fn redo_action(&mut self) {
        if let Some(redo_state) = self.state_history.redo() {
            self.state_history.undo_stack.push(self.full_state.clone());
            self.apply_diff(redo_state);
        }
    }

    fn save_state_before_action(&mut self) {
        self.state_history.before_action(&self.full_state);
    }

    fn apply_diff(&self, previous_state: FullState) {
        let diff_entries = self.full_state.diff(&previous_state);
        self.send_diff_updates(diff_entries);
    }

    fn send_diff_updates(&self, diff: TimeEntriesDiff) {
        if let Some(tx) = &self.command_tx {
            for entry in diff.to_upsert {
                let request = ApiRequest::FullEntryUpdate { entry };
                tx.send(Action::api_request_action(request)).unwrap()
            }
            for id in diff.to_delete {
                let request = ApiRequest::DeleteEntry { id };
                tx.send(Action::api_request_action(request)).unwrap()
            }
        }
    }

    fn set_time_entries(&mut self) {
        self.time_entry_container
            .set_time_entries(self.full_state.get_time_entries_for_day(self.current_day));

        self.set_note_for_entry(self.time_entry_container.get_selected_entry());
    }

    fn set_note_for_entry(&mut self, entry: Option<TimeEntry>) {
        match entry {
            Some(entry) => {
                self.notes.set_id(entry.id);
                let text = self
                    .full_state
                    .get_vms_for_day(self.current_day)
                    .and_then(|entries| entries.iter().find(|e| e.id == entry.id))
                    .map_or("".to_string(), |entry| entry.note.clone());

                self.notes.set_text(text);
            }
            None => {
                self.notes.set_text("".to_string());
                self.notes.set_id(0);
            }
        }
    }

    fn update_standup_for_current_day(&mut self) {
        match self.full_state.get_vms_for_day(self.current_day) {
            Some(current_days) => self.standup_container.aggregate_time_entries(
                current_days.as_slice(),
                self.full_state.charge_codes.as_slice(),
            ),
            None => self.standup_container.clear_entries(),
        }
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

    fn draw_crud_mode(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let top_bar_height = 3;
        let bottom_bar_height = 3;

        let remaining_space_height = area.height - top_bar_height - bottom_bar_height;

        let time_entry_container_height = (remaining_space_height as f32 * 0.75) as u16;
        let notes_height = remaining_space_height - time_entry_container_height;

        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(top_bar_height), // Fixed height for top bar
                Constraint::Length(time_entry_container_height), // 75% of remaining space for time_entry_container
                Constraint::Length(notes_height),                // Remaining space for notes
                Constraint::Length(bottom_bar_height),           // Fixed height for bottom bar
            ],
        )
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
        if self.swap_time_modal.is_active {
            self.swap_time_modal.draw(f, layout[1])?;
        }

        self.notes.draw(f, layout[2])?;

        let tooltips = vec![
            "Quit [q]",
            "Add [a]",
            "Delete [d]",
            "Play/Pause [Space]",
            "Code [c]",
            "Time [t]",
            "Swap [s]",
            "Undo [u]",
            "Redo [ctrl+r]",
        ];
        draw_tooltip_bar(f, layout[3], &tooltips);

        Ok(())
    }

    fn draw_standup_mode(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let top_bar_height = 3;
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(top_bar_height),
                Constraint::Min(0), // Remaining space for standup mode content
            ],
        )
        .split(area);

        self.top_bar.draw(f, layout[0])?;
        self.standup_container.draw(f, layout[1])?;
        Ok(())
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
        self.swap_time_modal.register_action_handler(tx.clone())?;

        // hacky: this initalizes the system with the right entry selected
        self.time_entry_container.send_index_action();

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

                    // time entry stuff
                    self.set_time_entries();
                    self.time_entry_container.set_index(0);
                    self.set_note_for_entry(self.time_entry_container.get_selected_entry());
                    self.time_entry_container.set_day(day);

                    // standup stuff
                    self.update_standup_for_current_day();
                }
                TTAct::UpdateSelectedEntry => {
                    self.save_state_before_action();
                    self.set_note_for_entry(self.time_entry_container.get_selected_entry());
                }
                TTAct::EditChargeCode(id) => {
                    self.save_state_before_action();
                    self.charge_code_modal.set_charge_code_id(id);
                    self.charge_code_modal.toggle();
                }
                TTAct::EditTime(time_action) => {
                    self.save_state_before_action();
                    self.time_edit_modal.set_time(time_action.millis);
                    self.time_edit_modal.set_entry_id(time_action.id);
                    self.time_edit_modal.toggle();
                }
                TTAct::UpdateMode(mode) => {
                    self.update_standup_for_current_day();
                    self.mode = mode;
                }
                TTAct::SwapTime(id) => {
                    self.save_state_before_action();
                    self.swap_time_modal.set_swap_from_id(id);

                    let other_entries: Vec<TimeEntry> = self
                        .full_state
                        .get_current_time_entries()
                        .into_iter()
                        .filter(|entry| entry.id != id)
                        .collect();

                    self.swap_time_modal.set_other_entries(other_entries);
                    self.swap_time_modal.toggle();
                }
                TTAct::SaveState => self.save_state_before_action(),
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
            return self.notes.handle_key_events(key);
        }
        if self.charge_code_modal.is_active {
            return self.charge_code_modal.handle_key_events(key);
        }
        if self.time_edit_modal.is_active {
            return self.time_edit_modal.handle_key_events(key);
        }
        if self.swap_time_modal.is_active {
            return self.swap_time_modal.handle_key_events(key);
        }

        // Handling global shortcuts
        match (key.code, key.modifiers) {
            (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                self.redo_action();
                return Ok(None);
            }
            (KeyCode::Char('u'), KeyModifiers::NONE) => {
                self.undo_action();
                return Ok(None);
            }
            (KeyCode::Char('q'), KeyModifiers::NONE) => {
                return Ok(Some(Action::UI(Quit)));
            }
            _ => {}
        }

        // If no modals are active and no global shortcuts are pressed, delegate to other components
        self.top_bar.handle_key_events(key)?;
        self.time_entry_container.handle_key_events(key)?;
        self.notes.handle_key_events(key)?;

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        match self.mode {
            Mode::Crud => self.draw_crud_mode(f, area),
            Mode::Standup => self.draw_standup_mode(f, area),
        }
    }
}
