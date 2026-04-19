use std::path::{PathBuf};
use domain::Song;
use miner::Miner; // Assuming your library is named 'miner'

fn assets_dir() -> PathBuf {
    PathBuf::from("tests")
        .join("assets")
}

fn bep_album() -> PathBuf {
    assets_dir().join("Bridging the Gap")
}

fn song_path() -> PathBuf {
    assets_dir().join("01 B.E.P. Empire - Hiphopde.com.mp3")
}

#[test]
fn test_init() {
    let album_path = bep_album();
    let miner = Miner::new(&album_path);
    assert_eq!(miner.root(), album_path.as_path());
    let invalid_miner = Miner::new("non_existent");
    assert_eq!(invalid_miner.root(), "non_existent");
}

#[test]
fn test_into_iter() {
    let album_path = bep_album();
    let miner1 = Miner::new(&album_path);
    let iter1 = miner1.start();
    assert_eq!(iter1.size(), 13);
    let miner2 = Miner::new(song_path());
    let iter2 = miner2.into_iter();
    assert_eq!(iter2.size(), 1);
    let miner3 = Miner::new("");
    let iter3 = miner3.into_iter();
    assert_eq!(iter3.size(), 0);
}

#[test]
fn test_iterator() {
    let miner = Miner::new(bep_album());
    let mut i:usize = 0;
    for _ in miner {
        i +=1;
    }
    assert_eq!(i, 13);
}

#[test]
fn test_mining_song() {
    let miner = Miner::new(song_path());
    let iter = miner.into_iter();
    assert!(
        iter.size() == 1, 
        "Incorrect number of elements{}", iter.size()
        );
    for (song_opt, path) in iter {
        assert!(
            path == song_path(),
            "Incorrect path: Expected: {} Given: {}", song_path().display(), path.display()
        );
        let song = song_opt.expect("Failed to read tags");
        assert_eq!(song.title, "B.E.P. Empire ");
        let artist = song.artists.first().unwrap();
        assert_eq!(artist, "Black Eyed Peas");
        assert_eq!(song.album, "Bridging the Gap");
        assert_eq!(song.track, Some(String::from("1")));
        assert_eq!(song.year, Some(String::from("2000")));
        assert_eq!(song.genre, Some(String::from("Hip-Hop/Rap")));
    }
}





