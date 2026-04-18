//! Miner library for extracting ID3 tags from MP3 files.
//!
//! This module provides an iterator-based API to scan directories for MP3 files
//! and extract their metadata tags.

use domain::Song;
use walkdir::WalkDir;
use id3::{Tag, TagLike};

/// Error type for tag extraction failures.
#[derive(Debug)]
pub enum TagError {
    /// Failed to read or parse the MP3 file.
    Parse(String),
    /// The file does not contain valid ID3 tags.
    NoTags(String),
    /// File I/O error.
    Io(std::io::Error),
}

impl std::fmt::Display for TagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagError::Parse(msg) => write!(f, "Parse error: {}", msg),
            TagError::NoTags(path) => write!(f, "No tags found in: {}", path),
            TagError::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for TagError {}

/// Main entry point for the miner.
///
/// # Example
///
/// ```ignore
/// use miner::Miner;
///
/// let miner = Miner::new("/path/to/music");
/// for result in miner {
///     match result {
///         Ok(song) => println!("Found: {}", song.title),
///         Err(path) => eprintln!("Invalid file: {}", path),
///     }
/// }
/// ```
pub struct Miner {
    root: String,
}

impl Miner {
    /// Creates a new Miner instance targeting the specified directory.
    ///
    /// # Arguments
    ///
    /// * `root` - Path to the directory to scan recursively
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }

    pub fn start(self) -> MinerIter {
        self.into_iter()
    }

}

/// Iterator for yielding songs from a directory scan.
///
/// Returns `Ok(Song)` for valid MP3 files or `Err(String)` for invalid paths.
pub struct MinerIter {
    paths: Vec<String>,
    current: usize,
}

impl MinerIter {
    /// Creates a new iterator from a list of file paths.
    pub fn new(path_files: Vec<String>) -> Self {
        Self {
            paths: path_files,
            current: 0,
        }
    }

    /// Returns the total number of paths in the iterator.
    pub fn size(&self) -> usize {
        self.paths.len()
    }

    /// Extracts ID3 tags from an MP3 file.
    ///
    /// Opens the file at `path_file`, reads the ID3v2.4 tags, and returns
    /// a `Song` struct with the extracted metadata.
    ///
    /// # Arguments
    ///
    /// * `path_file` - Path to the MP3 file
    ///
    /// # Returns
    ///
    /// * `Some(Song)` - If tags were successfully extracted
    /// * `None` - If there was an error parsing tags
    ///
    fn read_tags(path_file: &str) -> Option<Song> {
        let tag = Tag::read_from_path(path_file).ok()?;
        let title = 
            tag.title().unwrap_or("Unknown Title").to_string();
        let artist = 
            tag.artist().unwrap_or("Unknown Artist").to_string();
        let album = tag.album().unwrap_or("Unknown Album").to_string();
        let year = tag.year().map(|i| i.to_string()); 
        let genre = tag.genre().map(|g| g.to_string());
        Some(Song {
            title,
            artist,
            album,
            year,
            genre,
        })  
    }
}

/// Type alias for iterator results: Song on success, path string on invalid file.
type InvalidPath = String;

impl Iterator for MinerIter {
    type Item = Result<Song, InvalidPath>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.paths.len() {
            return None;
        }
        let current_file=&self.paths[self.current];
        self.current+=1;
        let res = 
            match MinerIter::read_tags(current_file) {
            Some(song) => Ok(song),
            None => Err(current_file.to_string())
            };
        Option::Some(res)
    }
}


// NOTE Make the process so it can iterate over levels in the 
// root directory to organize the songs, as the user wants. Use a tree?.
impl IntoIterator for Miner {
    type Item = Result<Song, InvalidPath>;
    type IntoIter = MinerIter;

    /// Consumes the Miner and returns an iterator that walks the directory.
    ///
    /// Scans `root` recursively and collects all file paths. The returned iterator
    /// will yield `Ok(Song)` for valid MP3 files and `Err(path)` for non-MP3 files.
    fn into_iter(self) -> Self::IntoIter {
        let mut paths = Vec::new();
        for entry in WalkDir::new(&self.root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(path_str) = path.to_str() {
                    paths.push(path_str.to_string());
                }
            }
        }

        MinerIter { paths, current: 0 }
    }
}
