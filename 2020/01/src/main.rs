#![deny(clippy::pedantic)]

use itertools::Itertools;
use std::env;
use std::fs;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let numbers: Vec<u32> = parse_input()?;
    let result = find_entries_adding_to_2020(numbers);

    println!("{}", result);
    Ok(())
}

fn find_entries_adding_to_2020(expenses: Vec<u32>) -> u64 {
    for (idx, entry) in expenses.iter().enumerate() {
        let (_, remaining_entries) = expenses.split_at(idx);

        for other_entry in remaining_entries {
            if entry + other_entry == 2020 {
                return (*entry as u64) * (*other_entry as u64);
            }
        }
    }

    return 0;
}

fn parse_input<ResultT>() -> Result<Vec<ResultT>, Error>
where
    ResultT: FromStr,
    ResultT::Err: 'static + std::error::Error,
{
    let input_file = env::args().nth(1).expect("input file name missing");

    let numbers = fs::read_to_string(input_file)?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<ResultT>, ResultT::Err>>()?;

    Ok(numbers)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_task_1_dummy_input() {
        let numbers = vec![1721, 979, 366, 299, 657, 1456];
        let result = find_entries_adding_to_2020(numbers);

        assert_eq!(514579, result);
    }
}
