type ArtistName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Artist {
    Solo(ArtistName),
    Various(Vec<ArtistName>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub title: String,
    pub artist: Option<Artist>,
    pub album: String,
    pub track: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
}
