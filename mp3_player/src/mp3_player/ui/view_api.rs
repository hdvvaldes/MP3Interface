use crate::mp3_player::controller::AppState;

pub trait PlayerView {
    
    fn setup(&mut self) -> Result<(), String>;

    fn render(&mut self, state: &AppState);

    fn display_message(&self, msg: &str);

    fn display_error(&self, reason: &str);
    
    fn teardown(&mut self) -> Result<(),String>;
}
