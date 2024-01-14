use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_tooltip_bar(f: &mut Frame<'_>, area: Rect, tooltips: &[&str]) {
    let mut spans = Vec::new();
    for (i, tooltip) in tooltips.iter().enumerate() {
        spans.push(Span::styled(
            *tooltip,
            Style::default().fg(Color::Black).bg(Color::Blue),
        ));
        // Don't add a separator after the last tooltip
        if i < tooltips.len() - 1 {
            spans.push(Span::raw(" "));
        }
    }

    let line = Line::from(spans);

    let paragraph = Paragraph::new(vec![line]).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}
