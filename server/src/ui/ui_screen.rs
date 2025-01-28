use super::ui_state::{CurrentScreen, UiState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_screen(f: &mut Frame, ui_state: &UiState) {
    //--------- MESSAGE HANDLING ----------

    // -------- MAIN LAYOUT ----------

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // -------- TITLE ----------

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        ui_state.title.clone(),
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Center)
    .block(title_block);

    // -------- MIDDLE ----------

    let instructions = navigation_instructions(ui_state);

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[1]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(middle_chunks[1]);

    let left_panel = Paragraph::new(Text::styled(
        ui_state.chat_messages.join("\n"),
        Style::default().fg(Color::Yellow),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title_top("Chat")
            .title_bottom(instructions.centered()),
    );

    let top_right = Paragraph::new(Text::styled(
        "USER LIST",
        Style::default().fg(Color::Yellow),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title_top("Users list"),
    );

    let bottom_right = Paragraph::new(Text::styled(
        "SYSTEM LOG",
        Style::default().fg(Color::Yellow),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title_top("System log"),
    );

    // -------- FOOTER ----------

    let footer = Paragraph::new(Span::styled(
        "MESSAGE WILL BE SEND HERE | Created by: Rafael Ramos",
        Style::default().fg(Color::Red),
    ))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, chunks[0]);
    f.render_widget(left_panel, middle_chunks[0]);
    f.render_widget(top_right, right_chunks[0]);
    f.render_widget(bottom_right, right_chunks[1]);
    f.render_widget(footer, chunks[2]);
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
