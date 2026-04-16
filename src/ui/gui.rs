
use crate::ui::PlayerView;
use crate::controller::AppState

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
