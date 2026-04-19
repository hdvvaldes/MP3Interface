#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub title: String,
    pub performer: String,
    pub album: String,
    pub track: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
}



