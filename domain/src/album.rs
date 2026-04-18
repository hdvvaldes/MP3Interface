#[derive(Debug, Clone, PartialEq)]
pub struct Album {
    pub id: Option<i64>,
    pub path: String,
    pub name: String,
    pub year: Option<i32>,
}

impl Album {
    pub fn new(name: String, path: String) -> Self {
        Self {
            id: None,
            path,
            name,
            year: None,
        }
    }
}
