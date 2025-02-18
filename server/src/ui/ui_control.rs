use super::ui_state::{CurrentScreen, UiState};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};

pub fn ui_control(ui_state: &mut UiState, key: Event, message: Option<String>) {
    if let Some(message) = message {
        if message.starts_with("UPDATE_USERS:") {
            let parts: Vec<&str> = message
                .strip_prefix("UPDATE_USERS:")
                .unwrap()
                .split(':')
                .collect();
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
            ui_state.chat_messages.push(message);
        }
    }

    if let Event::Key(KeyEvent { code, .. }) = key {
        match ui_state.current_screen {
            CurrentScreen::Main => match code {
                KeyCode::Char('q') => ui_state.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('c') => ui_state.toggle_screen(),
                _ => {}
            },
            CurrentScreen::Chatting => match code {
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
