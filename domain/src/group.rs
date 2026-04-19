#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
