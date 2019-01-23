use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    rc::Rc,
    str::FromStr,
};

use crate::{location::Location, player::Player, tile::Tile, Result};

#[derive(Debug, Clone)]
pub struct Map {
    pub size: (usize, usize),
    pub tiles: HashMap<Location, Tile>,
}

impl Map {
    /// Creates a list of all active players sorted after their current location on
    /// the map with an ordering that favors top before bottom and left before right.
    pub fn find_player(&self) -> Vec<Rc<RefCell<Player>>> {
        let mut tiles: Vec<&Location> = self.tiles.keys().collect();
        tiles.sort();

        tiles
            .into_iter()
            .map(|location| self.tiles.get(&location))
            .flatten()
            .map(Tile::as_player)
            .flatten()
            .collect()
    }

    pub fn free_adjacent(&self, location: &Location) -> Vec<Location> {
        location
            .adjacent()
            .into_iter()
            .filter(|location| {
                if let Some(tile) = self.tiles.get(location) {
                    tile.is_free()
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn is_free(&self, location: &Location) -> bool {
        match self.tiles.get(location) {
            Some(Tile::Floor) => true,
            _ => false,
        }
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let raw_tiles = input.lines().enumerate().flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, symbol)| (Location::new(x, y), symbol))
        });

        let mut tiles = HashMap::new();
        for (location, symbol) in raw_tiles {
            let mut tile = Tile::from_char(symbol)?;
            if let Tile::Player(player) = &mut tile {
                player.borrow_mut().location = location.clone();
            }

            tiles.insert(location, tile);
        }

        let width = tiles.keys().map(|location| location.x).max().unwrap_or(0);
        let height = tiles.keys().map(|location| location.y).max().unwrap_or(0);
        let size = (width + 1, height + 1);

        Ok(Map { size, tiles })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut entries: Vec<(&Location, &Tile)> = self.tiles.iter().collect();
        entries.sort_by_key(|(location, _)| *location);

        entries
            .chunks(self.size.0)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|(_, tile)| tile.to_string())
                    .collect::<String>()
            })
            .try_for_each(|row| writeln!(f, "{}", row))
    }
}
