
mod controller;
mod ui;

use controller::Player;

fn main() {
    let mp3_player = Player::new("TUI");
    mp3_player.start();
    let e_code:u8 = mp3_player.run();
    mp3_player.close(e_code);
}
