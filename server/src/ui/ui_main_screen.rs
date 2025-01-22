use super::ui_state::UiState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_main(f: &mut Frame, ui_state: &UiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    let title = Paragraph::new(Span::styled(
        "main screen - Press 'c' to chat, 'q' to quit",
        Style::default().fg(Color::Green),
    ))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, chunks[0]);
}
