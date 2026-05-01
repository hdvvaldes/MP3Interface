use crate::mp3_player::ui::view_api::{PlayerView, PlayerAction};
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
        // GUI rendering logic (e.g. using iced or egui)
        Ok(())
    }

    fn handle_events(&mut self) -> PlayerAction {
        PlayerAction::None
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
