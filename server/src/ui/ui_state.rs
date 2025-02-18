use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Chatting,
    Exiting,
}

pub struct UiState {
    pub current_screen: CurrentScreen,
    pub user_name: String,
    pub chat_messages: Vec<String>,
    pub title: String,
    pub users: HashMap<u32, String>,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            current_screen: CurrentScreen::Main,
            user_name: String::new(),
            chat_messages: Vec::new(),
            title: String::from("ShellWhisper"),
            users: HashMap::new(),
        }
    }

    pub fn toggle_screen(&mut self) {
        self.current_screen = match self.current_screen {
            CurrentScreen::Main => {
                self.title = String::from("ShellWhisper (Chat Mode)");
                CurrentScreen::Chatting
            }
            CurrentScreen::Chatting => {
                self.title = String::from("ShellWhisper");
                CurrentScreen::Main
            }
            CurrentScreen::Exiting => CurrentScreen::Exiting,
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.chat_messages.push(message);
    }
}
