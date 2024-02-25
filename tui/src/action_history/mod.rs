use shared_lib::models::full_state::FullState;

#[derive(Default)]
pub struct ActionHistory {
    pub undo_stack: Vec<FullState>,
    pub redo_stack: Vec<FullState>,
}

impl ActionHistory {
    pub fn before_action(&mut self, current_state: &FullState) {
        self.undo_stack.push(current_state.clone());
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<FullState> {
        self.undo_stack.pop()
    }

    pub fn redo(&mut self) -> Option<FullState> {
        self.redo_stack.pop()
    }
}
