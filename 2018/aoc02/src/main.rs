use std::{
    error::Error,
    io::{self, Read},
};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let checksum = calc_checksum(&input)?;
    println!("Boxes checksum: {}", checksum);

    Ok(())
}

fn calc_checksum(input: &str) -> Result<u32> {
    let ids: Vec<String> = input.split('\n').map(String::from).collect();
    let mut num_doubles = 0;
    let mut num_triples = 0;

    for id in ids {
        let mut contains_double = false;
        let mut contains_triple = false;

        for char in id.chars() {
            let num = id.chars().filter(|current| current == &char).count();
            match num {
                2 => contains_double = true,
                3 => contains_triple = true,
                _ => (),
            };
        }

        if contains_double {
            num_doubles += 1;
        }

        if contains_triple {
            num_triples += 1;
        }
    }

    Ok(num_doubles * num_triples)
}
