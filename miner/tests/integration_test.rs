use miner::Miner;
use std::fs::{self, File};
use tempfile::tempdir;

#[test]
fn test_full_workflow_collect_all_songs() {
    let dir = tempdir().unwrap();

    File::create(dir.path().join("song1.mp3")).unwrap();
    File::create(dir.path().join("song2.mp3")).unwrap();
    File::create(dir.path().join("readme.txt")).unwrap();

    let miner = Miner::new(dir.path().to_str().unwrap());
    let results: Vec<_> = miner.into_iter().collect();

    let songs: Vec<_> = results.iter().filter_map(|r| r.as_ref().ok()).collect();

    assert_eq!(songs.len(), 2);
    assert!(songs.iter().any(|s| s.title == "song1"));
    assert!(songs.iter().any(|s| s.title == "song2"));
}

#[test]
fn test_full_workflow_collect_errors() {
    let dir = tempdir().unwrap();

    File::create(dir.path().join("song.mp3")).unwrap();
    File::create(dir.path().join("document.txt")).unwrap();
    File::create(dir.path().join("image.png")).unwrap();

    let miner = Miner::new(dir.path().to_str().unwrap());
    let results: Vec<_> = miner.into_iter().collect();

    let errors: Vec<_> = results.iter().filter_map(|r| r.as_ref().err()).collect();

    assert_eq!(errors.len(), 2);
    assert!(errors.iter().any(|e| e.contains("document.txt")));
    assert!(errors.iter().any(|e| e.contains("image.png")));
}

#[test]
fn test_empty_directory_returns_empty() {
    let dir = tempdir().unwrap();
    let miner = Miner::new(dir.path().to_str().unwrap());

    let results: Vec<_> = miner.into_iter().collect();

    assert!(results.is_empty());
}

#[test]
fn test_recursive_traversal_nested_subdirs() {
    let dir = tempdir().unwrap();

    let subdir1 = dir.path().join("album1");
    let subdir2 = dir.path().join("album2");
    fs::create_dir(&subdir1).unwrap();
    fs::create_dir(&subdir2).unwrap();

    File::create(dir.path().join("root_song.mp3")).unwrap();
    File::create(subdir1.join("track1.mp3")).unwrap();
    File::create(subdir2.join("track2.mp3")).unwrap();

    let miner = Miner::new(dir.path().to_str().unwrap());
    let results: Vec<_> = miner.into_iter().collect();

    let songs: Vec<_> = results.iter().filter_map(|r| r.as_ref().ok()).collect();

    assert_eq!(songs.len(), 3);
    let paths: Vec<&str> = songs.iter().map(|s| s.path.as_str()).collect();
    assert!(paths.iter().any(|p| p.contains("root_song")));
    assert!(paths.iter().any(|p| p.contains("track1")));
    assert!(paths.iter().any(|p| p.contains("track2")));
}

#[test]
fn test_only_mp3_extensions_returned_as_songs() {
    let dir = tempdir().unwrap();

    File::create(dir.path().join("audio.mp3")).unwrap();
    File::create(dir.path().join("video.mp4")).unwrap();
    File::create(dir.path().join("image.jpg")).unwrap();
    File::create(dir.path().join("doc.pdf")).unwrap();

    let miner = Miner::new(dir.path().to_str().unwrap());
    let results: Vec<_> = miner.into_iter().collect();

    let songs = results.iter().filter(|r| r.is_ok()).count();
    let errors = results.iter().filter(|r| r.is_err()).count();

    assert_eq!(songs, 1);
    assert_eq!(errors, 3);
}

#[test]
fn test_deeply_nested_directory() {
    let dir = tempdir().unwrap();

    let deep_path = dir.path().join("a").join("b").join("c").join("d");
    fs::create_dir_all(&deep_path).unwrap();
    File::create(deep_path.join("deep_song.mp3")).unwrap();

    let miner = Miner::new(dir.path().to_str().unwrap());
    let results: Vec<_> = miner.into_iter().collect();

    let songs: Vec<_> = results.iter().filter_map(|r| r.as_ref().ok()).collect();
    assert_eq!(songs.len(), 1);
    assert!(songs[0].path.contains("deep_song"));
}

#[test]
fn test_nonexistent_directory_handling() {
    let miner = Miner::new("/nonexistent/path/that/does/not/exist");
    let results: Vec<_> = miner.into_iter().collect();

    assert!(results.is_empty());
}

#[test]
fn test_iterator_consumption_with_for_loop() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("song1.mp3")).unwrap();
    File::create(dir.path().join("song2.mp3")).unwrap();

    let miner = Miner::new(dir.path().to_str().unwrap());
    let mut count = 0;

    for result in miner {
        if result.is_ok() {
            count += 1;
        }
    }

    assert_eq!(count, 2);
}

#[test]
fn test_iterator_consumption_with_fold() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("a.mp3")).unwrap();
    File::create(dir.path().join("b.mp3")).unwrap();
    File::create(dir.path().join("c.mp3")).unwrap();

    let miner = Miner::new(dir.path().to_str().unwrap());

    let song_count = miner
        .into_iter()
        .fold(0, |acc, r| if r.is_ok() { acc + 1 } else { acc });

    assert_eq!(song_count, 3);
}

#[test]
fn test_mixed_success_and_failure_in_results() {
    let dir = tempdir().unwrap();

    File::create(dir.path().join("valid.mp3")).unwrap();
    File::create(dir.path().join("invalid.txt")).unwrap();
    File::create(dir.path().join("another.mp3")).unwrap();

    let miner = Miner::new(dir.path().to_str().unwrap());
    let results: Vec<_> = miner.into_iter().collect();

    let mut songs = Vec::new();
    let mut errors = Vec::new();

    for result in results {
        match result {
            Ok(song) => songs.push(song),
            Err(path) => errors.push(path),
        }
    }

    assert_eq!(songs.len(), 2);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("invalid.txt"));
}

#[test]
fn test_consuming_twice_from_same_source() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("song.mp3")).unwrap();

    let path = dir.path().to_str().unwrap();

    let first_miner = Miner::new(path);
    let first: Vec<_> = first_miner.into_iter().collect();

    let second_miner = Miner::new(path);
    let second: Vec<_> = second_miner.into_iter().collect();

    assert_eq!(first.len(), second.len());
}
