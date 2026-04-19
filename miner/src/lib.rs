//! Miner library for extracting ID3 tags from MP3 files.
//!
//! This module provides an iterator-based API to scan directories for MP3 files
//! and extract their metadata tags.

use std::path::{Path, PathBuf};

use domain::Song;

use walkdir::WalkDir;
use id3::{Tag, TagLike};

/// Main entry point for the miner.
///
/// # Example
///
/// use miner::Miner;
///
/// let miner = Miner::new("/path/to/music");
/// for (song, path) in miner {
///     match song {
///         Some() => println!("Found: {}", song.title),
///         _ => eprintln!("Invalid file: {}", path),
///     }
/// }
pub struct Miner {
    miner_root: PathBuf,
}

impl Miner {
    /// Creates a new Miner instance targeting the specified directory.
    ///
    /// # Arguments
    ///
    /// * `root` - Path to the directory to scan recursively
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self { 
            miner_root: root.as_ref().to_path_buf(), 
        }
    }

    /// Returns root directory
    ///
    /// # Returns
    ///
    /// * `miner_root`
    pub fn root(&self) -> &Path {
        self.miner_root.as_path()
    }
    
    /// Consumes the Miner and returns an iterator that walks the directory.
    /// Scans `root` recursively and collects all file paths. The returned iterator
    /// will yield `Ok(Song)` for valid MP3 files and `Err(path)` for non-MP3 files.
    pub fn start(self) -> MinerIter {
        self.into_iter()
    }

}
/// Iterator for yielding songs from a directory scan.
///
/// Returns `Ok(Song)` for valid MP3 files or `Err(String)` for invalid paths.
pub struct MinerIter {
    paths: Vec<PathBuf>,
    current: usize,
}

impl MinerIter {
    /// Creates a new iterator from a list of file paths.
    pub fn new(path_files: Vec<PathBuf>) -> Self {
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
    fn read_tags(&self) -> Option<Song> {
        let file_path = &self.paths[self.current];
        let tag = Tag::read_from_path(file_path).ok()?;
        let title = tag.title()
            .map(String::from)
            .unwrap_or_else(|| {
                file_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
            });
        let album = tag.album()
            .map(String::from)
            .unwrap_or_else(|| {
                file_path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|f| f.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "Unknown Album".to_string()) 
            });
        let track = tag.track().map(|t| t.to_string());
        let year = tag.year().map(|i| i.to_string()); 
        let genre = tag.genre().map(|g| g.to_string());
        let artists = tag.artists()
            .unwrap_or_default()
            .into_iter()
            .map(String::from)
            .collect();
        Some(Song { title, artists, album, track, year, genre })
    }
}

/// Type alias for iterator results.
/// First entry is None if tags can't be read
type MinerSong = (Option<Song>, PathBuf);

impl Iterator for MinerIter {
    type Item = MinerSong;

    fn next(&mut self) -> Option<Self::Item> {
        let current_file = self.paths.
            get(self.current)?;
        let song_opt = self.read_tags();
        self.current+=1;
        Some((song_opt, current_file.to_path_buf()))
    }
}


// NOTE Make the process so it can iterate over levels in the 
// root directory to organize the songs, as the user wants. Use a tree?.
impl IntoIterator for Miner {
    type Item = MinerSong;
    type IntoIter = MinerIter;

    /// Identical to Miner::start()
    fn into_iter(self) -> Self::IntoIter {
        let mut paths = Vec::new();
        for entry in WalkDir::new(&self.miner_root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.into_path();
            if path.is_file() {
                paths.push(path)
            }
        }
        MinerIter { paths, current: 0 }
    }
}
