pub enum CurrentScreen {
    Main,
    Chating,
    Exiting,
}

pub struct UiState {
    pub current_screen: CurrentScreen,
    pub user_name: String,
    pub chat_messages: Vec<String>,
    pub title: String,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            current_screen: CurrentScreen::Main,
            user_name: String::new(),
            chat_messages: Vec::new(),
            title: String::from("Main Screen"),
        }
    }

    pub fn toggle_screen(&mut self) {
        self.current_screen = match self.current_screen {
            CurrentScreen::Main => {
                self.title = String::from("Chat Screen");
                CurrentScreen::Chating
            }
            CurrentScreen::Chating => {
                self.title = String::from("Main Screen");
                CurrentScreen::Main
            }
            CurrentScreen::Exiting => CurrentScreen::Exiting,
        }
    }
}
