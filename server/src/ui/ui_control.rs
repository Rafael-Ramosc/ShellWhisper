use super::ui_state::{CurrentScreen, UiState};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};

pub fn ui_control(ui_state: &mut UiState, key: Event) {
    if let Event::Key(KeyEvent { code, .. }) = key {
        match ui_state.current_screen {
            CurrentScreen::Main => match code {
                KeyCode::Char('q') => ui_state.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('c') => ui_state.toggle_screen(),
                _ => {}
            },
            CurrentScreen::Chating => match code {
                KeyCode::Esc => ui_state.toggle_screen(),
                KeyCode::Char(c) => ui_state.chat_messages.push(c.to_string()),
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
            CurrentScreen::Exiting => {}
        }
    }
}
