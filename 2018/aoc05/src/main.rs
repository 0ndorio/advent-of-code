use std::{
    error::Error,
    io::{self, Read},
};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut polymer = Vec::new();
    io::stdin().read_to_end(&mut polymer)?;

    let polymer_length = run_polymer_reaction(&polymer)?;
    println!("{}", polymer_length);

    Ok(())
}

fn run_polymer_reaction(polymer: &[u8]) -> Result<usize> {
    let mut polymer = Vec::from(polymer);
    let mut reaction_input = vec![];

    while polymer.len() != reaction_input.len() {
        reaction_input = std::mem::replace(&mut polymer, vec![]);

        let mut units = reaction_input.iter().peekable();

        while let Some(&byte) = units.next() {
            match units.peek() {
                None => polymer.push(byte),
                Some(&&next_byte) => {
                    let unit_distance = byte.wrapping_sub(next_byte);
                    let should_react = unit_distance == 32 || unit_distance == 224;

                    if should_react {
                        units.next();
                    } else {
                        polymer.push(byte)
                    }
                }
            }
        }
    }

    Ok(polymer.len())
}