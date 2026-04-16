
mod mp3_player;

use mp3_player::App;

fn main() {
    let mp3_player = App::new("TUI");
    mp3_player.start();
    let e_code:u8 = mp3_player.run();
    mp3_player.close(e_code);
}
