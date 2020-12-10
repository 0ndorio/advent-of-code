use std::{env, fs};

use itertools::Itertools;

fn main() -> Result<(), Error> {
    let numbers = parse_input::<u64>()?;

    let weak_number =
        XMAS::find_weak_number(25, &numbers).ok_or("Couldn't determine weak number.")?;
    println!("The Weak Number is: {}", weak_number);

    let encryption_weakness = XMAS::find_encryption_weakness(weak_number, &numbers)
        .ok_or("Couldn't determine encryption weakness")?;
    println!("The Encryption Weakness is: {}", encryption_weakness);

    Ok(())
}

// ------------------------------------------------------------------------------
// XMAS
// ------------------------------------------------------------------------------

struct XMAS {}

impl XMAS {
    fn find_weak_number(preamble_size: usize, numbers: impl AsRef<[u64]>) -> Option<u64> {
        let numbers = numbers.as_ref();
        if preamble_size >= numbers.len() {
            return None;
        }

        for index in preamble_size..numbers.len() {
            let number = numbers[index];

            let preamble_start = index - preamble_size;
            let is_sum_of_preamble = numbers
                .iter()
                .skip(preamble_start)
                .take(preamble_size)
                .permutations(2)
                .map(|numbers| numbers.into_iter().sum::<u64>())
                .any(|value| value == number);

            if !is_sum_of_preamble {
                return Some(number);
            }
        }

        None
    }

    fn find_encryption_weakness(weak_number: u64, numbers: impl AsRef<[u64]>) -> Option<u64> {
        let numbers = numbers.as_ref();

        let (weak_index, _) = numbers
            .iter()
            .find_position(|&&number| number == weak_number)?;

        for index in 0..weak_index - 2 {
            for chunk_size in 2..weak_index {
                let chunk = numbers
                    .iter()
                    .skip(index)
                    .take(chunk_size)
                    .cloned()
                    .collect::<Vec<_>>();

                if weak_number == chunk.iter().sum::<u64>() {
                    let min_value = chunk.iter().min()?;
                    let max_value = chunk.iter().max()?;

                    return Some(min_value + max_value);
                }
            }
        }

        None
    }
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

type Error = Box<dyn std::error::Error>;

fn parse_input<ResultT>() -> Result<Vec<ResultT>, Error>
where
    ResultT: std::str::FromStr,
    ResultT::Err: Into<Error>,
{
    let input_file = env::args().nth(1).expect("input file name missing");

    fs::read_to_string(input_file)?
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEQUENCE: &str = "35
                            20
                            15
                            25
                            47
                            40
                            62
                            55
                            65
                            95
                            102
                            117
                            150
                            182
                            127
                            219
                            299
                            277
                            309
                            576";

    #[test]
    fn break_xmas_cipher() -> Result<(), Error> {
        let number_sequence = SEQUENCE
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<u64>, _>>()?;

        let weak_entry = XMAS::find_weak_number(5, &number_sequence).unwrap();
        assert_eq!(127, weak_entry);

        let encryption_weakness =
            XMAS::find_encryption_weakness(weak_entry, &number_sequence).unwrap();
        assert_eq!(62, encryption_weakness);

        Ok(())
    }
}
