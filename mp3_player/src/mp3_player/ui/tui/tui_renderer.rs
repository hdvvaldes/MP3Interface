use domain::Song;
use ratatui::{
    style::Style,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::mp3_player::controller::AppState;

pub fn render_init(f: &mut Frame) {
    let area = f.area();
    let paragraph = Paragraph::new("MP3 Player - Loading...")
        .block(Block::default().title("Init").borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

pub fn render_mining(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(f.area());

    let title = Paragraph::new("Scanning Directory...")
        .block(Block::default()
            .title("Mining Tags").borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let progress_text = format!("Scanned: {}/{} files", scanned, total);
            let progress = Paragraph::new(progress_text);
            f.render_widget(progress, chunks[1]);

            let file_text = format!("Current: {}", file);
            let current = Paragraph::new(file_text);
            f.render_widget(current, chunks[2]);
}

pub fn render_library(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(f.area());

    let song_list: Vec<ListItem> = if self.current_songs.is_empty() {
        vec![ListItem::new("No songs pub fn library")]
    } else {
        self.current_songs
            .iter()
            .enumerate()
            .map(|(i, song)| {
                ListItem::new(format!(
                        "{}. {} - {}",
                        i + 1,
                        song.title,
                        Self::get_artist_display(&song.artists)
                ))
            })
        .collect()
    };

    let list = List::new(song_list)
        .block(Block::default().title("Library").borders(Borders::ALL));
    f.render_widget(list, chunks[0]);

    let help = Paragraph::new("Controls:\n[Enter] Play\n[N] Next\n[P] Prev\n[S] Search\n[Q] Quit");
    f.render_widget(help, chunks[1]);
}
pub fn render_search(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.area());

    let query = "";
    let song = "";

    let search_prompt = Paragraph::new(format!("Search: {}", query))
        .block(Block::default().title("Search").borders(Borders::ALL));
    f.render_widget(search_prompt, chunks[0]);

    let query_lower = query.to_lowercase();

}

pub fn render_playing(area: ratatui::layout::Rect, f: &mut Frame, track_id: usize, is_paused: bool, songs: &[Song]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(area);

    let song = songs.get(track_id);
    let title = song.map(|s| s.title.as_str()).unwrap_or("Unknown");
    let artist = song.map(|s| Self::get_artist_display(&s.artists)).unwrap_or("Unknown");
    let album = song.map(|s| s.album.as_str()).unwrap_or("Unknown");

    let now_playing = Block::default()
        .title(format!("Now Playing: {} - {}", title, artist))
        .borders(Borders::ALL);
    f.render_widget(now_playing.title_style(Style::new().bold()), chunks[0]);
    f.render_widget(Paragraph::new(format!("Album: {}\nTrack: {}", album, track_id + 1)), chunks[0]);

    let status = if is_paused { "PAUSED" } else { "PLAYING" };
    let status_style = if is_paused {
        Style::new().red()
    } else {
        Style::new().green()
    };

    let controls = Paragraph::new(
        format!("Status: [{}]\n[Space] Pause/Resume\n[N] Next Track\n[P] Previous Track\n[Esc] Back to Library", status)
    ).style(status_style);
    f.render_widget(controls, chunks[1]);
}

fn render_songs<'a, I> (iterable : I) 
where 
    I: IntoIterator<Item= &'a Song> 
{

}



