mod game;
mod tile;
mod turn;

use crate::game::Game;

fn main() {
    let mut game = Game::new();

    game.play();
}
