use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    // layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

async fn draw_tui(charge_code: String) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    execute!(std::io::stdout(), EnterAlternateScreen)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().borders(Borders::ALL);
        let paragraph = Paragraph::new(charge_code)
            .block(block)
            .alignment(tui::layout::Alignment::Left);
        f.render_widget(paragraph, size);
    })?;

    execute!(std::io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let charge_code_data = fetch_charge_code(1).await?;
    draw_tui(charge_code_data).await?;

    Ok(())
}

async fn fetch_charge_code(id: i32) -> Result<String, reqwest::Error> {
    let url = format!("http://yourserveraddress/charge_codes/{}", id);
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}
