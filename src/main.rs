use std::io::{stdin, stdout, Write};

use crate::game::Game;

mod game;
mod tile;
mod turn;

fn main() {
    'game_loop:
    loop {
        let mut game = Game::new();

        game.play();

        loop {
            print!("Play again? ");

            let _ = stdout().flush();

            let mut input = String::new();

            match stdin().read_line(&mut input) {
                Ok(_) => {}

                Err(_) => continue
            }

            match input.trim().to_lowercase().as_str() {
                "yes" | "y" => continue 'game_loop,

                _ => break 'game_loop,
            }
        }
    }
}
