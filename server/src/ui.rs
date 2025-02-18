pub mod ui_control;
pub mod ui_screen;
pub mod ui_state;

use crate::server_state::State;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event},
    Terminal,
};
use std::sync::Arc;
use tokio::{select, sync::mpsc};
use ui_control::ui_control;
use ui_screen::render_screen;
use ui_state::{CurrentScreen, UiState};

pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: Arc<State>,
    mut message_rx: mpsc::Receiver<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ui_state = UiState::new();

    loop {
        terminal.draw(|f| match ui_state.current_screen {
            CurrentScreen::Main => render_screen(f, &ui_state),
            CurrentScreen::Chatting => render_screen(f, &ui_state),
            CurrentScreen::Exiting => {}
        })?;

        select! {
            event = tokio::spawn(async move { event::read() }) => {
                if let Ok(Ok(Event::Key(key))) = event {
                    ui_control(&mut ui_state, Event::Key(key), None);
                    if matches!(ui_state.current_screen, CurrentScreen::Exiting) {
                        break;
                    }
                }
            }
            Some(message) = message_rx.recv() => {
                if message.starts_with("UPDATE_USERS:") {
                    let parts: Vec<&str> = message.strip_prefix("UPDATE_USERS:").unwrap().split(':').collect();
                    if parts.len() == 2 {
                        if let Ok(id) = parts[0].parse::<u32>() {
                            ui_state.users.insert(id, parts[1].to_string());
                        }
                    }
                } else if message.starts_with("REMOVE_USER:") {
                    if let Ok(id) = message.strip_prefix("REMOVE_USER:").unwrap().parse::<u32>() {
                        ui_state.users.remove(&id);
                    }
                } else {
                    ui_state.add_message(message);
                }
            }
        }
    }

    Ok(())
}
