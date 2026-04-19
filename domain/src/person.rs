#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub stage_name: String,
    pub real_name: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
}
