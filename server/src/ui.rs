pub mod ui_control;
pub mod ui_screen;
pub mod ui_state;

use crate::server_state::State;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    Terminal,
};
use std::sync::Arc;
use ui_control::ui_control;
use ui_screen::render_screen;
use ui_state::{CurrentScreen, UiState};

pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: Arc<State>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ui_state = UiState::new();
    let mut receiver = state.message_sender.subscribe();

    loop {
        if let Ok(Ok(message)) =
            tokio::time::timeout(std::time::Duration::from_millis(10), receiver.recv()).await
        {
            ui_state.add_message(message);
        }

        terminal.draw(|f| match ui_state.current_screen {
            CurrentScreen::Main => render_screen(f, &ui_state),
            CurrentScreen::Chatting => render_screen(f, &ui_state),
            CurrentScreen::Exiting => {}
        })?;

        if let Event::Key(key) = event::read()? {
            ui_control(&mut ui_state, Event::Key(key));
            if matches!(ui_state.current_screen, CurrentScreen::Exiting) {
                break;
            }
        }
    }

    Ok(())
}
