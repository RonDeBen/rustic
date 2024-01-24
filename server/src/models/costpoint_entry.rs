#[derive(sqlx::FromRow)]
pub struct CostpointEntryRaw {
    pub charge_code: String,
    pub total_time_milliseconds: i64,
    pub entry_date: String,
}

#[derive(serde::Serialize)]
pub struct CostpointEntryVM {
    pub charge_code: String,
    pub hours: String,
    pub date: String,
}

impl From<CostpointEntryRaw> for CostpointEntryVM{
    fn from(value: CostpointEntryRaw) -> Self {
        Self {
            charge_code: value.charge_code,
            hours: milliseconds_to_quarter_hours(value.total_time_milliseconds),
            date: value.entry_date,
        }
    }
}

fn milliseconds_to_quarter_hours(milliseconds: i64) -> String {
    let hours = milliseconds as f64 / 3600000.0; // Convert milliseconds to hours
    let quarter_hours = (hours * 4.0).round() / 4.0; // Round to nearest quarter hour
    format!("{:.2}", quarter_hours) // Format with two decimal places
}

