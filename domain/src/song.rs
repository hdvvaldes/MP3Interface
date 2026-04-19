type ArtistName = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub title: String,
    pub artists: Vec<ArtistName>,
    pub album: String,
    pub track: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
}
