use crate::mp3_player::ui::{PlayerView, TUIView, GUIView}; 
use crate::mp3_player::controller::AppState;
use crate::mp3_player::controller::AppState::{Init};

pub struct Orchestrator{
    view: Box<dyn PlayerView>, 
    state: AppState, 
    
}

impl Orchestrator{

    pub fn new(view_opt: &str) -> Self {
        let selected_view: Box<dyn PlayerView> = 
            if view_opt == "TUI" {
                Box::new(TUIView::new())
            } else {
                Box::new(GUIView::new())
            };
        Orchestrator {
            view: selected_view,
            state: Init,
        }
    }

    pub fn start(&mut self) {
        self.view.setup();
    }

    pub fn run(&mut self) -> u8 {
        let mut running = true;
        while running {
            self.view.render(&self.state);

        }
         
        0
    }

    pub fn close(&self, exit_code:u8){
        match exit_code {
            _ => ()
        }
    }

}
