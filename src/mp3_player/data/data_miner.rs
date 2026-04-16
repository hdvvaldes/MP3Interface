use crate::domain::song;

pub struct Miner {
    _path_data : Vec<String>,
}

impl Miner {

    pub fn new(dir:&String) -> Self {
        return Self { 
            _path_data: vec![dir.to_string()],
        }
    }

    /*
     * Recollects all music path files 
     */

    pub fn start() -> Iter<Song> {


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

    pub fn hasNext() -> bool {
        return true;
    }

}

impl IntoIterator for Miner  {
    type Item = Song; 
}
