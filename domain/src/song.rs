#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub title: String,
    pub artist: String, // Just the name, extracted from ID3
    pub album: String,  // Just the name, extracted from ID3
    pub year: Option<String>,
    pub genre: Option<String>,
}
