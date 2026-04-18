#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub id: Option<i64>,
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            start_date: None,
            end_date: None,
        }
    }
}
