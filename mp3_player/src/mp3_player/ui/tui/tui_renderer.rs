use domain::Song;
use ratatui::{
    style::Style,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_init(f: &mut Frame, path: &str) {
    let area = f.area();
    let paragraph = Paragraph::new("MP3 Player - Loading...")
        .block(Block::default().title("Init").borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

pub fn render_mining (f: &mut Frame, scanned: usize, total: usize, current_file: String ) {
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

    let file_text = format!("Current: {}", current_file);
    let current = Paragraph::new(file_text);
    f.render_widget(current, chunks[2]);
}

pub fn render_library(f: &mut Frame, current_songs: &Vec<Song>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(f.area());

    let song_list: Vec<ListItem> = if current_songs.is_empty() {
        vec![ListItem::new("No songs pub fn library")]
    } else {
        current_songs
            .iter()
            .enumerate()
            .map(|(i, song)| {
                ListItem::new(format!(
                        "{}. {} - {}",
                        i + 1,
                        song.title,
                        ""))
            })
        .collect()
    };

    let list = List::new(song_list)
        .block(Block::default().title("Library").borders(Borders::ALL));
    f.render_widget(list, chunks[0]);

    let help = Paragraph::new("Controls:\n[Enter] Play\n[N] Next\n[P] Prev\n[S] Search\n[Q] Quit");
    f.render_widget(help, chunks[1]);
}
pub fn render_search(f: &mut Frame, query:&str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.area());

    let song = "";

    let search_prompt = Paragraph::new(format!("Search: {}", query))
        .block(Block::default().title("Search").borders(Borders::ALL));
    f.render_widget(search_prompt, chunks[0]);

    let query_lower = query.to_lowercase();

}

pub fn render_playing(f: &mut Frame, song: Song, paused: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(f.area());
    let now_playing = Block::default()
        .title( format!("Now Playing: {} - {}", song.title, 
                song.artists.iter()
                .fold(String::new(), |acc, a| acc + a)))
        .borders(Borders::ALL);
    f.render_widget(now_playing.title_style(Style::new().bold()), chunks[0]);
    f.render_widget(
        Paragraph::new(
            format!("Album: {}\nTrack: {}", song.album, 
                song.track.unwrap_or("unknown".to_string()))), 
        chunks[0]);
    let status = if paused { "PAUSED" } else { "PLAYING" };
    let status_style = if paused {
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

