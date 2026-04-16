use crate::mp3_player::domain::Song;
use crate::mp3_player::data::util_files::bfs_directory;
use walkdir::WalkDir;


pub struct Miner {
    _path_data : Vec<String>,
}

impl Miner {

    pub fn new(root:&String) -> Self {
        return Self { 
            _path_data: bfs_directory(root),
        }
    }

    /* 
     * Returns current file path as a song.
     * TODO:
     * if not an mp3 file 
     * throw an exception and manage it later
     *  
     */

    pub fn next_song() -> Song {
        return ;
    }

}

fn get_files(root: &String) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root).into_iter() {
        match entry {
            Ok(dir) => files.push(dir.file_type),
            _ => return vec![]
        } 
    }
    files
}

struct MinerIter;

type InvalidSongs = String;

impl Iterator for MinerIter {
    type Item = Song;
    
    fn next(&mut self) -> Option<Self::Item> {
        
    }
}

impl IntoIterator for Miner  {
    type Item = Song; 
    type IntoIter = MinerIter;

    fn into_iter(self) -> Self::IntoIter {
        
    }
}
