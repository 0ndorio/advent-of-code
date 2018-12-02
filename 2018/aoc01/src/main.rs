use std::{
    error::Error,
    io::{self, Read},
    str::FromStr,
};

type Result<ContentT> = std::result::Result<ContentT, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let frequency_update = calc_frequency(&input)?;

    println!("Frequency update: {}", frequency_update);

    Ok(())
}

fn calc_frequency(input: &str) -> Result<i32> {
    let frequency_update: i32 = input.split('\n').flat_map(i32::from_str).sum();

    Ok(frequency_update)
}
