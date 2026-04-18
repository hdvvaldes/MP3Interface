#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub id: Option<i64>,
    pub id_performer: Option<i64>,
    pub id_album: Option<i64>,
    pub path: String,
    pub title: String,
    pub track: Option<i32>,
    pub year: Option<i32>,
    pub genre: Option<String>,
}

impl Song {
    pub fn new(title: String, path: String) -> Self {
        Self {
            id: None,
            id_performer: None,
            id_album: None,
            path,
            title,
            track: None,
            year: None,
            genre: None,
        }
    }
}
