use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    X,
    O,
    Empty(usize),
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        if let Tile::Empty(_) = self {
            true
        } else {
            false
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::X => 'X',

            Tile::O => 'O',

            Tile::Empty(n) => char::from_digit(*n as u32, 10).unwrap(),
        })
    }
}