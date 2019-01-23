mod game;
mod location;
mod map;
mod player;
mod tile;

use std::{
    error::Error,
    io::{self, Read},
    str::FromStr,
};

use crate::{game::Game, map::Map, player::Race};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = Map::from_str(&input)?;
    let game = Game::new(map);

    let (num_turns, winner, hp_lfeft) = game.clone().run();
    println!(
        "{:?} won after {} total turns with {} hp left.",
        winner, num_turns, hp_lfeft
    );

    let (num_turns, winner, hp_lfeft) = cheat_until_elves_win(&input)?;
    println!(
        "{:?} won after {} total turns with {} hp left.",
        winner, num_turns, hp_lfeft
    );

    Ok(())
}

/// Runs the simulation until all elves survive the fight.
pub fn cheat_until_elves_win(input: &str) -> Result<(u32, Race, u32)> {
    let mut elf_attack_power = 3;

    loop {
        elf_attack_power += 1;

        let map = Map::from_str(&input)?;
        let mut game = Game::new(map);
        game.set_elf_attack_power(elf_attack_power);

        let total_num_elves = game.count_elves();

        let result = game.run();
        if game.count_elves() == total_num_elves {
            return Ok(result);
        }
    }
}
