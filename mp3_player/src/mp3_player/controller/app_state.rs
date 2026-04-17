
pub enum AppState {
    NON_STARTED,

    Home,
    Playing { track_id: usize, is_paused: bool },
    Search { query: String },
    Library,
    // Your future mining process fits perfectly here:
    MiningTags { scanned: usize, total: usize, current_file: String },
}

