mod mp3_player;

use mp3_player::App;

fn main() {
    let mut mp3_player = App::new("TUI");
    if let Err(e) = mp3_player.start() {
        eprintln!("Failed to initialize application: {}", e);
        std::process::exit(1);
    };
    mp3_player.run();
}
