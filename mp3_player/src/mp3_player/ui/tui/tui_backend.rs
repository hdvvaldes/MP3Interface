use std::rc::Rc;
use std::cell::RefCell;

use std::sync::mpsc::Sender;

use crossterm::event::KeyCode;

use crate::mp3_player::controller::AppState;
use crate::mp3_player::ui::{PlayerAction, PlayerView};

pub trait ActionUI {
    fn manage_press(&self, code: KeyCode) -> PlayerAction;
}

impl<F> ActionUI for F
where
    F: Fn(KeyCode) -> PlayerAction,
{
    fn manage_press(&self, code: KeyCode) -> PlayerAction {
        self(code)
    }
}

pub struct UIHandler {
    transmitter: Option<Sender<PlayerAction>>,
    actions: Box<dyn ActionUI>,
}

impl UIHandler {
    pub fn new() -> Self {
        UIHandler {
            transmitter: None,
            actions: UIHandler::text_input(),
        }
    }


    pub fn start(&mut self) -> Result<(), String> {
        self.visuals.setup().map_err(|e| e.to_string())
    }

    pub fn handle_events(&mut self, state: &AppState) -> PlayerAction {
        let Some(code) = self.visuals.user_keystrokes() else {
            return PlayerAction::None;
        };
        self.actions.manage_press(code)
    }

    pub fn render(&mut self, state: &AppState) -> Result<(), String> {
        self.visuals.render(state)
    }

    pub fn close_tui(&mut self) {
        let _ = self.visuals.teardown();
    }
}

pub fn action_input() -> Box<dyn ActionUI> {
    Box::new(move |code: KeyCode| match code {
        KeyCode::Char(':') => PlayerAction::Search,
        KeyCode::Char('q') => PlayerAction::Quit,
        KeyCode::Char('n') => PlayerAction::Next,
        KeyCode::Char('p') => PlayerAction::Previous,
        KeyCode::Char(' ') => PlayerAction::Pause,
        KeyCode::Enter => PlayerAction::Play,
        KeyCode::Esc => PlayerAction::Quit,
        _ => PlayerAction::None,
    })
}
