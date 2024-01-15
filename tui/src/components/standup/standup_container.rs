use std::collections::HashMap;

use crate::{
    api_client::models::{charge_code::ChargeCode, time_entry::TimeEntryVM},
    components::Component,
    tui::Frame,
};
use color_eyre::eyre::Result;
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, widgets::{Paragraph, Block, Borders, Wrap}, style::Style};

#[derive(Default)]
pub struct StandupContainer {
    standup_entries: Vec<StandupEntry>,
}

pub struct StandupEntry {
    charge_code: Option<ChargeCode>,
    rounded_minutes: u16,
    notes: String,
}

impl StandupContainer {
    pub fn aggregate_time_entries(&mut self, entries: &[TimeEntryVM], charge_codes: &[ChargeCode]) {
        let mut aggregation: HashMap<i32, Vec<&TimeEntryVM>> = HashMap::new();

        for entry in entries {
            let key = match &entry.charge_code {
                Some(code) => code.id,
                None => -1,
            };

            aggregation.entry(key).or_default().push(entry);
        }

        let mut standup_entries: Vec<StandupEntry> = Vec::default();

        for (k, v) in aggregation {
            let notes = v
                .iter()
                .map(|entry| entry.note.clone())
                .collect::<Vec<String>>()
                .join("\n\n");

            standup_entries.push(StandupEntry {
                charge_code: get_code_from_id(charge_codes, k),
                rounded_minutes: sum_to_nearest_quarter_hour(v.as_slice()),
                notes,
            });
        }

        self.standup_entries = standup_entries;
    }
}

fn get_code_from_id(codes: &[ChargeCode], id: i32) -> Option<ChargeCode> {
    if id == -1 {
        return None;
    }

    codes.iter().find(|code| code.id == id).cloned()
}

fn sum_to_nearest_quarter_hour(entries: &[&TimeEntryVM]) -> u16 {
    let total_time_millis: i64 = entries.iter().map(|entry| entry.total_time).sum();
    let total_minutes = total_time_millis / 1000 / 60; // Convert milliseconds to minutes
    ((total_minutes as f64 / 15.0).round() * 15.0) as u16 // Round to nearest quarter hou
}

impl Component for StandupContainer {
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.standup_entries
                    .iter()
                    .map(|_| Constraint::Length(rect.height / self.standup_entries.len() as u16))
                    .collect::<Vec<_>>(),
            )
            .split(rect);

        for (i, entry) in self.standup_entries.iter().enumerate() {
            let title = format!(
                "{} ({}) - {:.2} hrs",
                entry.charge_code.as_ref().map_or("", |cc| &cc.alias),
                entry.charge_code.as_ref().map_or("", |cc| &cc.code),
                entry.rounded_minutes as f64 / 60.0 // Convert minutes to hours
            );

            let paragraph = Paragraph::new(entry.notes.as_str())
                .block(Block::default().title(title).borders(Borders::ALL))
                .wrap(Wrap { trim: true })
                .style(Style::default());

            f.render_widget(paragraph, chunks[i]);
        }

        Ok(())
    }
}
