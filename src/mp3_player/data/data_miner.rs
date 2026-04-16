use crate::mp3_player::domain::Song;
use walkdir::WalkDir;

pub struct Miner {
    root: String,
}

impl Miner {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }

    fn get_tags(path_file: &str) -> Song {
        // Placeholder for actual ID3 tag extraction
        // For now, it returns an empty Song (as defined in domain/song.rs)
        Song {}
    }
}

pub struct MinerIter {
    paths: Vec<String>,
    current: usize,
}

impl MinerIter {
    pub fn new(path_files: Vec<String>) -> Self {
        Self {
            paths: path_files,
            current: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.paths.len()
    }
}

type InvalidPath = String;

impl Iterator for MinerIter {
    type Item = Result<Song, InvalidPath>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.paths.len() {
            let path = &self.paths[self.current];
            self.current += 1;
            // In a real implementation, get_tags might return a Result
            Some(Ok(Miner::get_tags(path)))
        } else {
            None
        }
    }
}

impl IntoIterator for Miner {
    type Item = Result<Song, InvalidPath>;
    type IntoIter = MinerIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut paths = Vec::new();

        for entry in WalkDir::new(&self.root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "mp3" {
                        if let Some(path_str) = path.to_str() {
                            paths.push(path_str.to_string());
                        }
                    }
                }
            }
        }

        MinerIter {
            paths,
            current: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_miner_new() {
        let miner = Miner::new("/tmp");
        assert_eq!(miner.root, "/tmp");
    }

    #[test]
    fn test_miner_iter_size() {
        let paths = vec!["a.mp3".to_string(), "b.mp3".to_string()];
        let iter = MinerIter::new(paths);
        assert_eq!(iter.size(), 2);
    }

    #[test]
    fn test_miner_iter_next() {
        let paths = vec!["a.mp3".to_string()];
        let mut iter = MinerIter::new(paths);
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_miner_traversal() {
        let dir = tempdir().expect("Failed to create temp dir");
        let file_path = dir.path().join("test.mp3");
        File::create(&file_path).expect("Failed to create temp file");

        let miner = Miner::new(dir.path().to_str().unwrap());
        let items: Vec<_> = miner.into_iter().collect();

        assert_eq!(items.len(), 1);
        assert!(items[0].is_ok());
    }
}
