use crossterm::event::KeyCode;

use crate::mp3_player::ui::view_api::{PlayerView};
use crate::mp3_player::controller::AppState;

pub struct GUIView;

impl GUIView {
    pub fn new() -> GUIView {
        GUIView
    }
}

impl PlayerView for GUIView {
    fn setup(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn render(&mut self, _state: &AppState) -> Result<(), String> {
        Ok(())
    }

    fn user_keystrokes() -> Option<KeyCode> {
        return None 
    }

    fn display_error(&self, reason: &str) {
        eprintln!("GUI Error: {}", reason);
    }

    fn display_message(&self, msg: &str) {
        println!("GUI Info: {}", msg);
    }

    fn teardown(&mut self) -> Result<(), String> {
        Ok(())
    }
    

}
