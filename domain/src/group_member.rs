#[derive(Debug, Clone, PartialEq)]
pub struct GroupMember {
    pub id_person: i64,
    pub id_group: i64,
}

impl GroupMember {
    pub fn new(id_person: i64, id_group: i64) -> Self {
        Self {
            id_person,
            id_group,
        }
    }
}
