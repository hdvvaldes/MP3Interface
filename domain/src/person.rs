#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub id: Option<i64>,
    pub stage_name: String,
    pub real_name: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
}

impl Person {
    pub fn new(stage_name: String) -> Self {
        Self {
            id: None,
            stage_name,
            real_name: None,
            birth_date: None,
            death_date: None,
        }
    }
}
