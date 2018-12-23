use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::{self, Read},
};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;
type PolymerUnit = u8;

fn main() -> Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let polymer_length = run_polymer_reaction(&input)?;
    println!("{}", polymer_length);

    let (symbol, polymer_length) = find_broken_unit_type(&input)?;
    println!(
        "Reached minimal polymer length of {} as we removed {}",
        polymer_length, symbol as char
    );

    Ok(())
}

fn run_polymer_reaction(polymer: &[PolymerUnit]) -> Result<usize> {
    let mut polymer = Vec::from(polymer);

    loop {
        let reaction_input_length = polymer.len();

        let mut reaction_input = polymer.into_iter().peekable();
        polymer = vec![];

        while let Some(byte) = reaction_input.next() {
            match reaction_input.peek() {
                None => polymer.push(byte),
                Some(&next_byte) => {
                    let unit_distance = byte.wrapping_sub(next_byte);
                    let should_react = unit_distance == 32 || unit_distance == 224;

                    if should_react {
                        reaction_input.next();
                    } else {
                        polymer.push(byte)
                    }
                }
            }
        }

        if polymer.len() == reaction_input_length {
            break;
        }
    }

    Ok(polymer.len())
}

fn find_broken_unit_type(polymer: &[PolymerUnit]) -> Result<(PolymerUnit, usize)> {
    let mut filter_results = HashMap::new();

    let unit_types: HashSet<(PolymerUnit, PolymerUnit)> = polymer
        .iter()
        .map(|&unit| (unit.to_ascii_lowercase(), unit.to_ascii_uppercase()))
        .collect();

    for (symbol_lowercase, symbol_uppercase) in unit_types {
        let filtered_polymer: Vec<PolymerUnit> = polymer
            .iter()
            .filter(|&&byte| byte != symbol_lowercase && byte != (symbol_uppercase))
            .cloned()
            .collect();

        let polymer_length = run_polymer_reaction(&filtered_polymer)?;
        filter_results.insert(symbol_lowercase, polymer_length);
    }

    let (&symbol, &polymer_length) = filter_results
        .iter()
        .min_by_key(|(_, &polymer_length)| polymer_length)
        .expect("This map must always contain as many values as unit types.");

    Ok((symbol, polymer_length))
}
