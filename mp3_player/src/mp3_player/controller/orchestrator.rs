use domain::Song;

use crate::mp3_player::ui::{PlayerView, TUIView, GUIView}; 
use crate::mp3_player::controller::AppState;
use crate::mp3_player::controller::AppState::{Init};
use crate::mp3_player::ui::PlayerAction;

pub struct Orchestrator {
    view: Box<dyn PlayerView>, 
    state: AppState, 
    music_playing: bool
}

impl Orchestrator {

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
            music_playing: false
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.view.setup().map_err(|e| e.to_string())
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            if let Err(e) = self.view.render(&self.state) {
                self.view.display_error(&e);
                break;
            }
            match self.view.handle_events() {
                PlayerAction::Play(id) => self.play_song(id),
                PlayerAction::Pause => self.pause_song(),
                PlayerAction::Resume => self.resume_song(),
                PlayerAction::Next => self.next_song(),
                PlayerAction::Previous => self.previous_song(),
                PlayerAction::Search(s) => self.search_song(s),
                PlayerAction::Quit => running = false,
                PlayerAction::None => ()
            }
        }
        self.close(0);
    }

    fn play_song(&self, song: Song) {

    }

    fn pause_song(&self) {

    }

    fn resume_song(&self) {

    }

    fn next_song(&self) {

    }

    fn previous_song(&self) {

    }

    fn search_song(&self, prop: String) {

    }

    fn quit(&self) {

    }

    fn toggle_pause(&mut self) {
        self.music_playing = !self.music_playing;
    }

    pub fn close(&mut self, exit_code: u8) {
        let _ = self.view.teardown();
        match exit_code {
            _ => ()
        }
    }
}
