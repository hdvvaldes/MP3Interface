use std::sync::mpsc;
use std::thread;

use domain::Song;

use crate::mp3_player::ui::{PlayerView, TUIView, GUIView}; 
use crate::mp3_player::controller::AppState;
use crate::mp3_player::controller::AppState::{Init};
use crate::mp3_player::ui::{PlayerAction, UIHandler};

use miner::Miner;

pub struct Orchestrator {
    view: UIHandler,
    player_action : Option<PlayerAction>, 
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
            view: UIHandler::new(selected_view),
            player_action: None,
            state: Init,
            music_playing: false
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.view.start();
        let (tx, rx) = mpsc::channel::<PlayerAction>();
        let handle = thread::spawn(|| self.view.run());

        self.view.events()

        let path = self.view.ask_user();
        let miner = Miner::new();

        Ok(())
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
                PlayerAction::None => (),
                PlayerAction::Command(a) => self.evaluate_comm(),
            }
        }
        self.close();
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

    fn evaluate_comm(&self) {

    }


    fn toggle_pause(&mut self) {
        self.music_playing = !self.music_playing;
    }

    fn close(&mut self) {
        self.view.close_tui();
    }

}
