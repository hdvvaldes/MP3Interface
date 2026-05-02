use std ::thread;

use std::{sync::mpsc::{self, Receiver, Sender}};

use crossterm::event::{KeyCode, KeyEvent};

use crate::mp3_player::ui::{PlayerAction, PlayerView};


pub struct UIHandler {
    visuals: Box<dyn PlayerView>,
    transmitter: Option<Sender<PlayerAction>>,
}

impl UIHandler {
    
    pub fn new( view : Box<dyn PlayerView>) -> Self {
        UIHandler { visuals: view , transmitter: None} 
    }

    fn manage_press(&self) -> PlayerAction {
        let Some(code) = self.visuals.user_keystrokes() else {
            return PlayerAction::None
        };
        match code {
            KeyCode::Char(':') => 
                return PlayerAction::Search(self.get_query()),
            KeyCode::Char('q') | KeyCode::Char('Q') => 
                return PlayerAction::Quit,
            KeyCode::Char('n') | KeyCode::Char('N') => 
                return PlayerAction::Next,
            KeyCode::Char('p') | KeyCode::Char('P') => 
                return PlayerAction::Previous,
            KeyCode::Char(' ') => return PlayerAction::Pause,
            KeyCode::Enter => 
                return PlayerAction::Play(self.selected_song()),
            KeyCode::Esc => return PlayerAction::Quit,
            _ => return PlayerAction::None,
        }
    }

     
    pub fn start(&mut self) -> Result<Receiver<PlayerAction>, String> {
        self.visuals.setup().map_err(|e| e.to_string())?;
        let (tx, rx) = mpsc::channel::<PlayerAction>();
        self.transmitter = Some(tx);
        let _ = thread::spawn(move | | tx.send(PlayerAction::None));
        Ok(rx)
    }

    pub fn run(&mut self) -> PlayerAction {
        self.manage_press()
    }


    fn get_query(&self) -> String{
        String::new()
    }

    fn selected_song(&self) -> Song {
        return 
    }

    pub fn close_tui(&self)  {
        let _ = self.view.teardown();
    }


}


