use shared_lib::models::full_state::FullState;

pub mod monitor_orchistrator;
pub mod actions;

pub enum MonitorActionResult {
    StopTimer(i32),
    DeleteEntry(i32),
    Multiple(Vec<MonitorActionResult>),
}

pub trait MonitorAction {
    fn execute(&self, full_state: &FullState) -> Option<MonitorActionResult>;
}


