use std::{cell::RefCell, rc::Rc};

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use super::tui_renderer;
use crate::mp3_player::ui::{ActionUI, view_api::{PlayerAction, PlayerView}};
use crate::mp3_player::controller::AppState;
use crate::mp3_player::domain::Song;

type TuiTerminal = Option<Terminal<CrosstermBackend<std::io::Stdout>>>;

pub struct TUIView {
    terminal: TuiTerminal,
    current_songs: Vec<Song>,
}

impl TUIView {

    pub fn new() -> Self {
        TUIView {
            terminal: None,
            current_songs: Vec::new()
        }
    }

    fn selected_song(&self) -> Song {
        Song { title: String::new(),
            artists: Vec::new(),
            album: String::new(),
            track: None,
            year: None, 
            genre: None }
    }

    fn get_query(&self) -> String {
        String::new()
    }

    fn get_artist_display(artists: &[String]) -> &str {
        artists.first().map(|s| s.as_str()).unwrap_or("Unknown")
    }

    pub fn set_songs(&mut self, songs: Vec<Song>) {
        self.current_songs = songs;
    }

    pub fn text_input(buffer: &str) -> Box<dyn ActionUI> {
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


}

impl PlayerView for TUIView {

    fn setup(&mut self) -> Result<(), String> {
        enable_raw_mode().map_err(|e| e.to_string())?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen).map_err(|e| e.to_string())?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).map_err(|e| e.to_string())?;
        self.terminal = Some(terminal);
        Ok(())
    }

    fn render(&mut self, state: &AppState) -> Result<(), String> {
        let terminal = self.terminal.as_mut().ok_or(
            "The view has not been initialized")?;
        terminal.draw(|f| {
            match state {
                AppState::Init{path} => 
                    tui_renderer::render_init(f, path),
                AppState::MiningTags {scanned, total, current_file} =>  
                    tui_renderer::render_mining (
                        f, *scanned, *total, current_file.clone()),
                AppState::Library => 
                    tui_renderer::render_library(f, &self.current_songs),
                AppState::Search {query} => 
                    tui_renderer::render_search(f, query),
                AppState::Playing{song, is_paused} =>
                    tui_renderer::render_playing (
                        f, song.clone(), *is_paused),
            }
        }).map(|_| ()).map_err(|e| e.to_string())
    }

    fn handle_events(&mut self, _state: &AppState) -> PlayerAction {
        if let Some(code) = self.user_keystrokes() {
            match code {
                KeyCode::Char('q') | KeyCode::Esc => PlayerAction::Quit,
                KeyCode::Char(':')  => PlayerAction::Search,
                KeyCode::Char(' ') => PlayerAction::Pause,
                KeyCode::Enter => PlayerAction::Play,
                KeyCode::Char('n') => PlayerAction::Next,
                KeyCode::Char('p') => PlayerAction::Previous,
                _ => PlayerAction::None,
            }
        } else {
            PlayerAction::None
        }
    }


    fn user_keystrokes(&mut self) -> Option<KeyCode>  {
        if let Ok(Event::Key(key)) = event::read() {
            return Some(key.code)
        }
        None

    }

    fn display_error(&self, reason: &str) {
        eprintln!("Error: {}", reason);
    }

    fn display_message(&self, msg: &str) {
        println!("Info: {}", msg);
    }

    fn teardown(&mut self) -> Result<(), String> {
        disable_raw_mode().map_err(|e| e.to_string())?;
        execute!(
            std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

}
