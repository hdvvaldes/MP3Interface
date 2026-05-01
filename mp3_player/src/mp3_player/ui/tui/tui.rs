use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::mp3_player::ui::{tui::tui_renderer, view_api::{PlayerAction, PlayerView}};
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

    // Move this to controller and implemente strategy pattern
    fn manage_press(&self, key: KeyEvent) -> PlayerAction{
        match key.code {
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
                AppState::Init => 
                    tui_renderer::render_init(f),
                AppState::MiningTags {scanned, total, current_file} =>  
                    tui_renderer::render_mining (
                        f, *scanned, *total, current_file.clone()),
                AppState::Library => 
                    tui_renderer::render_library(f, &self.current_songs),
                AppState::Search {..} => 
                    tui_renderer::render_search(f),
                AppState::Playing{song, is_paused} =>
                    tui_renderer::render_playing (
                        f, song.clone(), *is_paused),
            }
        }).map(|_| ()).map_err(|e| e.to_string())
    }

    fn handle_events(&mut self) -> PlayerAction {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                return self.manage_press(key)
            }
        }
        PlayerAction::None
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
