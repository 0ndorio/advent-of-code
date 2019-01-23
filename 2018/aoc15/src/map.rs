use std::collections::BTreeMap;

use crate::{location::Location, tile::Tile};

#[derive(Debug, Clone)]
pub struct Map {
    pub size: (usize, usize),
    pub tiles: BTreeMap<Location, Tile>,
}

impl Map {
    pub fn new(tiles: BTreeMap<Location, Tile>) -> Self {
        let width = tiles.keys().map(|location| location.x).max().unwrap_or(0);
        let height = tiles.keys().map(|location| location.y).max().unwrap_or(0);
        let size = (width + 1, height + 1);

        Map { size, tiles }
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
