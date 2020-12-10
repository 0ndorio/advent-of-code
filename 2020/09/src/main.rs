use std::{env, fs};

use itertools::Itertools;

fn main() -> Result<(), Error> {
    let numbers = parse_input::<u64>()?;

    let weak_number = XMAS::find_weak_number(25, &numbers).unwrap_or_default();
    println!("The Weak Number is: {}", weak_number);

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
    use itertools::__std_iter::Iterator;

    #[test]
    fn find_weak_byte_in_sequence() -> Result<(), Error> {
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

        let number_sequence = SEQUENCE
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<u64>, _>>()?;

        let weak_entry = XMAS::find_weak_number(5, &number_sequence).unwrap();
        assert_eq!(127, weak_entry);

        Ok(())
    }
}
