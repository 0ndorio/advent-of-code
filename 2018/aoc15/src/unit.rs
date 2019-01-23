use std::fmt::{self, Display, Formatter};

use crate::{location::Location, Result};

#[derive(Debug)]
pub struct Unit {
    pub attack_power: u32,
    pub hit_points: u32,
    pub race: Race,
    pub location: Location,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Race {
    Elf,
    Gnome,
}

impl Unit {
    pub fn from_char(symbol: char) -> Result<Self> {
        let race = Race::from_char(symbol)?;

        Ok(Unit {
            race,
            attack_power: 3,
            hit_points: 200,
            location: Location { x: 0, y: 0 },
        })
    }

    /// Returns true if the target dies through the attack.
    pub fn attack(&self, target: &mut Unit) -> bool {
        if target.hit_points < self.attack_power {
            target.hit_points = 0;
        } else {
            target.hit_points -= self.attack_power;
        }

        !target.alive()
    }

    pub fn alive(&self) -> bool {
        self.hit_points > 0
    }
}

impl Race {
    fn from_char(symbol: char) -> Result<Self> {
        let race = match symbol {
            'E' => Race::Elf,
            'G' => Race::Gnome,
            _ => return Err(format!("Unknown race symbol: {}", symbol))?,
        };

        Ok(race)
    }

    fn to_char(self) -> char {
        match self {
            Race::Elf => 'E',
            Race::Gnome => 'G',
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.race.to_char().fmt(f)
    }
}
