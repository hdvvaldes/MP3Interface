use crossterm::event::KeyCode;

use crate::mp3_player::controller::AppState;

pub enum PlayerAction {
    Play,
    Pause,
    Resume,
    Next,
    Previous,
    Search,
    Quit,
    Input(String),
    None,
}

pub trait PlayerView {
    fn setup(&mut self) -> Result<(), String>;

    fn render(&mut self, state: &AppState) -> Result<(), String>;

    fn user_keystrokes(&mut self) -> Option<KeyCode>;

    fn display_message(&self, msg: &str);

    fn display_error(&self, reason: &str);

    fn teardown(&mut self) -> Result<(), String>;

    fn handle_events(&mut self, state: &AppState) -> PlayerAction;
}
