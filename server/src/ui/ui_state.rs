pub enum CurrentScreen {
    Main,
    Chating,
    Exiting,
}

pub struct UiState {
    pub current_screen: CurrentScreen,
    pub user_name: String,
    pub chat_messages: Vec<String>,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            current_screen: CurrentScreen::Main,
            user_name: String::new(),
            chat_messages: Vec::new(),
        }
    }

    pub fn toggle_screen(&mut self) {
        self.current_screen = match self.current_screen {
            CurrentScreen::Main => CurrentScreen::Chating,
            CurrentScreen::Chating => CurrentScreen::Main,
            CurrentScreen::Exiting => CurrentScreen::Exiting,
        }
    }
}
