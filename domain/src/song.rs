use crate::person::Person;
use crate::group::Group;
use crate::album::Album;

#[derive(Debug, Clone, PartialEq)]
pub enum Performer {
    Single(Person), 
    Multiple(Group),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub title: String,
    pub performer: Performer,
    pub album: Album,
    pub track: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
}



