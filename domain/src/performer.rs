#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PerformerType {
    Person = 0,
    Group = 1,
    Unknown = 2,
}

impl From<i32> for PerformerType {
    fn from(value: i32) -> Self {
        match value {
            0 => PerformerType::Person,
            1 => PerformerType::Group,
            _ => PerformerType::Unknown,
        }
    }
}

impl From<PerformerType> for i32 {
    fn from(pt: PerformerType) -> Self {
        pt as i32
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Performer {
    pub id: Option<i64>,
    pub id_type: PerformerType,
    pub name: String,
}

impl Performer {
    pub fn new(name: String, id_type: PerformerType) -> Self {
        Self {
            id: None,
            id_type,
            name,
        }
    }
}
