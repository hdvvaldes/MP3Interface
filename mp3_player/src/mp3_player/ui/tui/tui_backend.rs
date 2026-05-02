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
    visuals: Box<dyn PlayerView>,
    transmitter: Option<Sender<PlayerAction>>,
    actions: Box<dyn ActionUI>,
}

impl UIHandler {
    pub fn new(view: Box<dyn PlayerView>) -> Self {
        UIHandler {
            visuals: view,
            transmitter: None,
            actions: UIHandler::text_input(),
        }
    }

    fn text_input() -> Box<dyn ActionUI> {
        let buffer = Rc::new(RefCell::new(String::new()));
        Box::new(move |code: KeyCode| {
            let mut current_text = buffer.borrow_mut();
            match code {
                KeyCode::Char(c) => {
                    current_text.push(c);
                    PlayerAction::None
                }

                KeyCode::Backspace => {
                    current_text.pop();
                    PlayerAction::None
                }

                KeyCode::Enter => {
                    let final_string = current_text.clone();
                    current_text.clear();
                    PlayerAction::Input(final_string)
                }
                _ => PlayerAction::None,
            }
        })
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
