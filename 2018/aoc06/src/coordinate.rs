use std::{collections::HashMap, error::Error, str::FromStr};

use crate::{Identifier, Result};

pub type Grid = HashMap<Coordinate, DistanceInformation>;
pub type DistanceInformation = HashMap<Identifier, Distance>;
pub type Distance = u32;

#[derive(PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
    pub border_tile: bool,
}

impl Coordinate {
    pub fn distance(&self, target: &Coordinate) -> Distance {
        let x_distance = self.x.max(target.x) - self.x.min(target.x);
        let y_distance = self.y.max(target.y) - self.y.min(target.y);

        x_distance + y_distance
    }
}

impl FromStr for Coordinate {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let mut values = input.split(',').map(str::trim).flat_map(str::parse::<u32>);

        let x = values
            .next()
            .ok_or_else(|| "Couldn't parse x-value.".to_string())?;
        let y = values
            .next()
            .ok_or_else(|| "Couldn't parse y-value.".to_string())?;

        let border_tile = false;

        Ok(Self { x, y, border_tile })
    }
}

pub fn find_owner(distances: &DistanceInformation) -> Option<Identifier> {
    let owner = distances.iter().fold(vec![], |mut owner, next| {
        match owner.first() {
            None => owner.push(next),
            Some(old) => {
                if next.1 == old.1 {
                    owner.push(next);
                } else if next.1 < old.1 {
                    owner.clear();
                    owner.push(next);
                }
            }
        }

        owner
    });

    if owner.len() != 1 {
        None
    } else {
        owner.first().map(|(identifier, _)| **identifier)
    }
}

pub fn total_distance(distances: &DistanceInformation) -> u32 {
    distances.values().sum()
}
