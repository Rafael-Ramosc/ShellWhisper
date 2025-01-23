use super::ui_state::{CurrentScreen, UiState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_screen(f: &mut Frame, ui_state: &UiState) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        ui_state.title.clone(),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    let instructions = navigation_instructions(ui_state);

    let middle = Paragraph::new(Text::styled(
        "Chat window",
        Style::default().fg(Color::Yellow),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title_bottom(instructions.centered()),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let footer_chuncks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    let navigation_text =
        Paragraph::new("Navigation").block(Block::default().borders(Borders::ALL));

    let creator_text = Paragraph::new(Span::styled(
        "Created by: Rafael Ramos",
        Style::default().fg(Color::Red),
    ))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, chunks[0]);
    f.render_widget(middle, chunks[1]);
    f.render_widget(navigation_text, footer_chuncks[0]);
    f.render_widget(creator_text, footer_chuncks[1]);
}

fn navigation_instructions(ui_state: &UiState) -> Line<'_> {
    match ui_state.current_screen {
        CurrentScreen::Main => Line::from(vec![
            Span::raw(" Send message "),
            Span::styled("<ENTER>", Style::default().fg(Color::Blue).bold()),
            Span::raw(" Chat mode "),
            Span::styled("<C>", Style::default().fg(Color::Blue).bold()),
            Span::raw(" Quit "),
            Span::styled("<Q>", Style::default().fg(Color::Blue).bold()),
        ]),
        CurrentScreen::Chatting => Line::from(vec![
            Span::raw(" Go back "),
            Span::styled("<ESC>", Style::default().fg(Color::Blue).bold()),
            Span::raw(" Send message "),
            Span::styled("<Enter>", Style::default().fg(Color::Blue).bold()),
            Span::raw(" Quit "),
            Span::styled("<Q>", Style::default().fg(Color::Blue).bold()),
        ]),
        CurrentScreen::Exiting => Line::styled("Exiting...", Style::default().fg(Color::Red)),
    }
}
