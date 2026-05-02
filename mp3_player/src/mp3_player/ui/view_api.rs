use crossterm::event::KeyCode;
use domain::Song;

use crate::mp3_player::controller::AppState;

pub enum PlayerAction {
    Play(Song),
    Pause,
    Resume,
    Next,
    Previous,
    Search(String),
    Quit,
    Command(String),
    None,
}

pub trait PlayerView {
    fn setup(&mut self) -> Result<(), String>;

    fn render(&mut self, state: &AppState) -> Result<(), String>;

    fn user_keystrokes(&mut self) -> Option<KeyCode>;

    fn display_message(&self, msg: &str);

    fn display_error(&self, reason: &str);

    fn teardown(&mut self) -> Result<(), String>;
    
}
