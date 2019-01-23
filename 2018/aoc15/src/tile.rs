use std::fmt::{self, Display, Formatter};

use crate::Result;

#[derive(Debug, Clone)]
pub enum Tile {
    Wall,
    Floor,
}

impl Tile {
    pub fn from_char(symbol: char) -> Result<Self> {
        let tile = match symbol {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            _ => return Err(format!("Unknown map tile: {}", symbol))?,
        };

        Ok(tile)
    }

    pub fn is_free(&self) -> bool {
        if let Tile::Floor = self {
            true
        } else {
            false
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let symbol = match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
        };

        symbol.fmt(f)
    }
}
