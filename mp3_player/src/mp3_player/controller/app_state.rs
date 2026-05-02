use domain::Song;

pub enum AppState {
    Init {path: String},
    MiningTags { scanned: usize, total: usize, current_file: String },
    Library,
    Searching { query: String },
    Playing { song: Song , is_paused: bool },
}

