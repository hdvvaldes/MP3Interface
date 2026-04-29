
pub enum AppState {
    Init,
    MiningTags { scanned: usize, total: usize, current_file: String },
    Library,
    Search { query: String },
    Playing { track_id: usize, is_paused: bool },
}

