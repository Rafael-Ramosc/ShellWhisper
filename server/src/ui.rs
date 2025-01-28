pub mod ui_control;
pub mod ui_screen;
pub mod ui_state;

use crate::handles::message::Message;
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
                    ui_control(&mut ui_state, Event::Key(key));
                    if matches!(ui_state.current_screen, CurrentScreen::Exiting) {
                        break;
                    }
                }
            }
            Some(formatted_message) = message_rx.recv() => {
                ui_state.add_message(formatted_message);
            }
        }
    }

    Ok(())
}
