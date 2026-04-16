use crate::mp3_player::ui::PlayerView;
use crate::mp3_player::controller::AppState;

pub struct GUIView;

impl GUIView {
    pub fn new() -> GUIView {
        GUIView
    }
}


impl PlayerView for GUIView {



    fn render(&mut self, state: &AppState) {
        
    }

    fn display_error(&self, reason: &str) {
        
    }

    fn display_message(&self, msg: &str) {
        
    }



}
