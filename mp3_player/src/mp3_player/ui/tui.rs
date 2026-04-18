use crate::mp3_player::ui::view_api::PlayerView;
use crate::mp3_player::controller::AppState;

pub struct TUIView;

impl TUIView {

    pub fn new() -> TUIView {
        TUIView
    }

}

impl PlayerView for TUIView {

    fn render(&mut self, state: &AppState) {
        
    } 

    fn display_error(&self, reason: &str) {
        

    }

    fn display_message(&self, msg: &str) {
        
    }
}



