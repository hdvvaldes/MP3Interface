use crate::controller::AppState;

pub trait PlayerView {
    fn render(&mut self, state: &AppState);
    
    // You can keep generic overlays separate
    fn display_message(&self, msg: &str);
    fn display_error(&self, reason: &str);
}
