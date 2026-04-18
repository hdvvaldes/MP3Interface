use crate::mp3_player::ui::{PlayerView, TUIView, GUIView}; 
use crate::mp3_player::controller::AppState;
use crate::mp3_player::controller::AppState::NON_STARTED;

pub struct Player {
    _view: Box<dyn PlayerView>, 
    _state: AppState, 
    
}

impl Player {

    pub fn new(view_opt: &str) -> Player {
        
        let selected_view: Box<dyn PlayerView> = 
            if view_opt == "TUI" {
            Box::new(TUIView::new())
        } else {
            Box::new(GUIView::new())
        };
        Player {
            _view: selected_view,
            _state: NON_STARTED,
        }
    }

    pub fn start(&self) {


    }

    pub fn run(&self) -> u8 {

        0
    }

    fn is_running(&self) -> bool {
        false
    }

    pub fn close(&self, exit_code:u8){
        match exit_code {
            _ => ()
        }
    }

}




