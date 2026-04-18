use std::path::{PathBuf};
use miner::Miner; // Assuming your library is named 'miner'

fn create_miner() -> Miner {
    let album_path = PathBuf::from("tests")
        .join("assets")
        .join("Bridging the Gap");
    Miner::new(&album_path)
}

#[test]
fn test_init() {
    let album_path = PathBuf::from("tests")
        .join("assets")
        .join("Bridging the Gap");
    let miner = Miner::new(&album_path);
    assert_eq!(miner.root(), album_path.as_path());

    let invalid_miner = Miner::new("non_existent");
    assert_eq!(invalid_miner.root(), "non_existent");
}

#[test]
fn test_into_iter() {
    let miner = create_miner(); 
    let iter = miner.start();
    assert_eq!(iter.size(), 13);
}

//NOTE 13 is a magic number lol
#[test]
fn test_iterator() {
    let miner = create_miner();
    let mut i:usize = 0;
    for _ in miner {
        i +=1;
    }
    assert_eq!(i, 13);
}

#[test]
fn test_mining_song() {
    let album_path = PathBuf::from("tests")
        .join("assets")
        .join("Bridging the Gap");
    let miner = Miner::new(&album_path);
    let mut miner_iter = miner.into_iter();
    // 2. Act: Search the iterator for this specific file
    let target_filename = "01 B.E.P. Empire - Hiphopde.com.mp3";
    
    // `.find()` consumes the iterator until it finds a path containing our target filename
    let mined_result = miner_iter.find(|(_, path)| path.contains(target_filename));

    // 3. Assert: Verify the file was found in the directory
    assert!(
        mined_result.is_some(), 
        "Could not find the target MP3 file in the directory!"
    );

    let (song_opt, path) = mined_result.unwrap();

    // Verify the tags were successfully extracted (Not None)
    assert!(
        song_opt.is_some(), 
        "Failed to parse ID3 tags for the file: {}", path
    );

    let song = song_opt.unwrap();

    // 4. Assert the Pure Domain values!
    // Note: Mp3-sharing sites often tamper with ID3 tags. If this test fails, 
    // it means the website altered the tags, and you should update these 
    // expected values to match whatever `println!("{:?}", song)` outputs!
    assert_eq!(song.title, "B.E.P. Empire ");
    assert_eq!(song.artist, "Black Eyed Peas"); 
    assert_eq!(song.album, "Bridging the Gap");
    assert_eq!(song.year, Some(String::from("2000")));
}


