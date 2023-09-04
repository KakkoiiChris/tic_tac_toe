use std::fmt::{Display, Formatter};

use crate::tile::Tile;

#[derive(PartialEq)]
pub enum Turn {
    X,
    O,
}

impl Turn {
    pub fn from(tile: Tile) -> Turn {
        match tile {
            Tile::X => Turn::X,

            Tile::O => Turn::O,

            _ => panic!("INVALID!")
        }
    }

    pub fn next(&self) -> Turn {
        match self {
            Turn::X => Turn::O,

            Turn::O => Turn::X,
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Turn::X => 'X',

            Turn::O => 'O',
        })
    }
}