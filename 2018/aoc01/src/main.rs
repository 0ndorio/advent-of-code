use std::{
    collections::HashSet,
    error::Error,
    io::{self, Read},
    str::FromStr,
};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let frequency_update = calc_final_frequency(&input)?;
    println!("Frequency update: {}", frequency_update);

    let duplicated_frequency = find_duplicated_frequency(&input)?;
    println!("Duplicated frequency: {}", duplicated_frequency);

    Ok(())
}

fn calc_final_frequency(input: &str) -> Result<i32> {
    let frequency_update: i32 = input.split('\n').flat_map(i32::from_str).sum();
    Ok(frequency_update)
}

fn find_duplicated_frequency(input: &str) -> Result<i32> {
    let frequency_update: Vec<i32> = input.split('\n').flat_map(i32::from_str).collect();

    let mut frequency_list = HashSet::new();
    let mut last_frequency = 0;

    for change in frequency_update.iter().cycle() {
        last_frequency += change;

        if !frequency_list.insert(last_frequency) {
            break;
        }
    }

    Ok(last_frequency)
}
