use std::{
    error::Error,
    io::{self, Read},
};

use regex::Regex;

mod list;
use crate::list::{Circle, Node};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

#[derive(Debug)]
struct Marble {
    value: u32,
}

impl Marble {
    fn new(value: u32) -> Self {
        Marble { value }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let game_rules_re = Regex::new(r"^(?P<player>\d+) players; .+ (?P<marbles>\d+) points")?;
    let captures = game_rules_re
        .captures(&input)
        .ok_or_else(|| format!("Couldn't parse game setup from {}", input))?;

    let num_players = captures["player"].parse::<u32>()?;
    let num_marbles = captures["marbles"].parse::<u32>()?;

    let max_score = play_marbles(num_players, num_marbles)?;
    println!("The winning Elf's score in round one is: {}", max_score);

    let max_score = play_marbles(num_players, num_marbles * 100)?;
    println!("The winning Elf's score in round two is: {}", max_score);

    Ok(())
}

fn play_marbles(num_players: u32, num_marbles: u32) -> Result<u64> {
    let mut scores = vec![0; num_players as usize];

    let root = Node::new(Marble::new(0));
    let mut circle = Circle::new(root);

    for value in 1..=num_marbles {
        if value % 23 != 0 {
            circle.move_forward(1);
            circle.insert(Marble::new(value));
        } else {
            circle.move_backwards(7);
            let removed_marble = circle.remove()?;

            let player = value % num_players;
            scores[player as usize] += value + removed_marble.value;
        }
    }

    let max_score = scores
        .iter()
        .max()
        .ok_or("We need at least one elf to play a game.")?;

    Ok(u64::from(*max_score))
}
