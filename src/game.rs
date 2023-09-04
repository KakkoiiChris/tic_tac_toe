use core::array::from_fn;
use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};

use crate::tile::Tile;
use crate::turn::Turn;

#[derive(PartialEq)]
pub enum State {
    Play,
    Win(Turn),
    Tie,
}

pub struct Game {
    state: State,
    turn: Turn,
    board: [Tile; 9],
}

impl Game {
    const WINS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6]
    ];

    pub fn new() -> Game {
        Game {
            state: State::Play,
            turn: Turn::X,
            board: from_fn(|i| Tile::Empty(i + 1)),
        }
    }

    pub fn play(&mut self) {
        println!("TIC TAC TOE!\n");

        while self.state == State::Play {
            println!("{self}");

            let index = self.choose_tile();

            match self.board[index] {
                Tile::Empty(_) => {
                    self.board[index] = match self.turn {
                        Turn::X => Tile::X,

                        Turn::O => Tile::O,
                    }
                }

                _ => {
                    println!("Tile {} is already occupied!", index + 1);

                    continue;
                }
            }

            self.state = self.get_next_state();

            self.turn = self.turn.next();
        }

        match &self.state {
            State::Win(turn) => println!("Player {turn} wins!"),

            State::Tie => println!("It's a tie."),

            _ => {}
        }
    }

    fn choose_tile(&self) -> usize {
        let choice: usize = loop {
            print!("> ");

            let _ = stdout().flush();

            let mut input = String::new();

            match stdin().read_line(&mut input) {
                Ok(_) => {}

                Err(e) => {
                    println!("Input Error: {}", e);
                    continue;
                }
            }

            match input.trim().parse::<usize>() {
                Ok(num) => break num,

                Err(e) => {
                    println!("{e}");
                    continue;
                }
            }
        };

        choice - 1
    }

    fn get_next_state(&self) -> State {
        'outer:
        for indices in Game::WINS.iter() {
            let first = self.board[indices[0]];

            for i in 1..indices.len() {
                if self.board[indices[i]] != first {
                    continue 'outer;
                }
            }

            return State::Win(Turn::from(first));
        }

        if self.board.iter().all(|tile| !tile.is_empty()) {
            return State::Tie;
        }

        State::Play
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player {}'s Turn:\n", self.turn)?;

        for r in 0..3 {
            for c in 0..3 {
                let i = (r * 3) + c;

                write!(f, "{}", self.board[i])?;

                if c < 2 {
                    write!(f, " | ")?;
                }
            }

            if r < 2 {
                write!(f, "\n--|---|--\n")?;
            }
        }

        Ok(())
    }
}