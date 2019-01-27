use std::{
    error::Error,
    ops::{Index, IndexMut},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::Result;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct State(pub u8, pub u8, pub u8, pub u8);

impl Default for State {
    fn default() -> Self {
        Self(0, 0, 0, 0)
    }
}

impl Index<u8> for State {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Invalid register index {}", index),
        }
    }
}

impl IndexMut<u8> for State {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Invalid register index {}", index),
        }
    }
}

impl FromStr for State {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        lazy_static! {
            static ref STATE_RE: Regex = Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]",)
                .expect("Predefined state regex failed to compile.");
        };

        let captures = STATE_RE
            .captures(&input)
            .ok_or_else(|| format!("Couldn't parse input entry line: {}.", input))?;

        Ok(State(
            captures[1].parse()?,
            captures[2].parse()?,
            captures[3].parse()?,
            captures[4].parse()?,
        ))
    }
}
