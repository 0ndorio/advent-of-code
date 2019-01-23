use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

use crate::{player::Player, Result};

#[derive(Debug, Clone)]
pub enum Tile {
    Wall,
    Floor,
    Player(Rc<RefCell<Player>>),
}

impl Tile {
    pub fn from_char(symbol: char) -> Result<Self> {
        let tile = match symbol {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            race @ 'E' | race @ 'G' => {
                let player = Player::from_char(race)?;
                Tile::Player(Rc::new(RefCell::new(player)))
            }
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

    pub fn as_player(&self) -> Option<Rc<RefCell<Player>>> {
        if let Tile::Player(player) = self {
            return Some(Rc::clone(player));
        }

        None
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let symbol = match self {
            Tile::Floor => '.',
            Tile::Player(player) => player.borrow().to_char(),
            Tile::Wall => '#',
        };

        symbol.fmt(f)
    }
}
