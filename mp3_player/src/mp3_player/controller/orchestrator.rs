use std::sync::mpsc::Receiver;

use crate::mp3_player::controller::AppState;
use crate::mp3_player::controller::AppState::Init;
use crate::mp3_player::ui::{PlayerAction, PlayerView, TUIView};

use miner::Miner;

pub struct Orchestrator {
    view: Box<dyn PlayerView>,
    receiver: Option<Receiver<PlayerAction>>,
    player_action: Option<PlayerAction>,
    state: AppState,
    music_playing: bool,
}

impl Orchestrator {
    pub fn new(view_opt: &str) -> Self {
        let selected_view: Box<dyn PlayerView> = Box::new(TUIView::new());
        Orchestrator {
            view: selected_view,
            receiver: None,
            player_action: None,
            state: Init { path: String::new() },
            music_playing: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.view.setup()?;
        self.view.render(&self.state)?;
        let _miner = Miner::new("");
        Ok(())
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            if let Err(e) = self.view.render(&self.state) {
                self.view.display_error(&e);
                break;
            }

            match self.view.handle_events(&self.state) {
                PlayerAction::Search => self.search_song(),
                PlayerAction::Play => self.play_song(),
                PlayerAction::Pause => self.pause_song(),
                PlayerAction::Resume => self.resume_song(),
                PlayerAction::Next => self.next_song(),
                PlayerAction::Previous => self.previous_song(),
                PlayerAction::Quit => running = false,
                PlayerAction::Input(_s) => (),
                PlayerAction::None => (),
            }
        }
        self.close();
    }

    fn play_song(&self) {}

    fn pause_song(&self) {}

    fn resume_song(&self) {}

    fn next_song(&self) {}

    fn previous_song(&self) {}

    fn search_song(&self) {}

    fn toggle_pause(&mut self) {
        self.music_playing = !self.music_playing;
    }

    fn close(&mut self) {
        let _ = self.view.teardown();
    }
}
