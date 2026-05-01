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
    None,
}

pub trait PlayerView {
    fn setup(&mut self) -> Result<(), String>;

    fn render(&mut self, state: &AppState) -> Result<(), String>;

    fn handle_events(&mut self) -> PlayerAction;

    fn display_message(&self, msg: &str);

    fn display_error(&self, reason: &str);

    fn teardown(&mut self) -> Result<(), String>;
}
