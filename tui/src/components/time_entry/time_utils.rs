pub fn format_millis(millis: &i64) -> String {
    let hours = millis / 3_600_000;
    let minutes = (millis % 3_600_000) / 60_000;
    let seconds = (millis % 60_000) / 1_000;
    let milliseconds = millis % 1_000;

    format!(
        "{:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    )
}
