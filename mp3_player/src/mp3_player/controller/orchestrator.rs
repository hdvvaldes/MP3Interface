use crate::mp3_player::controller::AppState;
use crate::mp3_player::controller::AppState::{Init, Searching}
use crate::mp3_player::ui::{ActionUI, PlayerAction, PlayerView, TUIView};

use miner::Miner;

pub struct Orchestrator {
    view: Box<dyn PlayerView>,
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
            match self.get_handler().manage_press() {
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

    fn get_handler(&self) -> Box<dyn ActionUI>{
        match self.state {
            Init { path } => TUIView::text_input(&path),
            Searching { query } => TUIView::text_input(&query),
            _ => TUIView::

        }

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
