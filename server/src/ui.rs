pub mod ui_state;

use crate::server_state::State;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    Terminal,
};
use std::sync::Arc;
use ui_state::{CurrentScreen, UiState};

pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: Arc<State>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ui_state = UiState::new();

    loop {
        terminal.draw(|f| match ui_state.current_screen {
            CurrentScreen::Main => {}
            CurrentScreen::Chating => {}
            CurrentScreen::Exiting => {}
        })?;

        if let Event::Key(key) = event::read()? {
            match ui_state.current_screen {
                CurrentScreen::Main => {
                    match key.code {
                        KeyCode::Char('q') => {
                            ui_state.current_screen = CurrentScreen::Exiting;
                            break;
                        }
                        KeyCode::Char('c') => {
                            ui_state.toggle_screen();
                        }
                        // TODO: need to handle other screens
                        _ => {}
                    }
                }
                CurrentScreen::Chating => match key.code {
                    KeyCode::Esc => {
                        ui_state.toggle_screen();
                    }
                    KeyCode::Char(c) => {
                        ui_state.chat_messages.push(c.to_string());
                    }
                    KeyCode::Backspace => {
                        ui_state.chat_messages.pop();
                    }
                    KeyCode::Enter => {
                        if !ui_state.chat_messages.is_empty() {
                            ui_state.chat_messages.clear();
                        }
                    }

                    _ => {}
                },
                CurrentScreen::Exiting => {
                    break;
                }
            }
        }
    }

    Ok(())
}
