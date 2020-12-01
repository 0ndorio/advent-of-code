#![deny(clippy::pedantic)]
#![feature(min_const_generics)]

use itertools::Itertools;
use std::convert::TryInto;
use std::env;
use std::fs;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let numbers: Vec<u32> = parse_input()?;

    let [first, second] = find_entries_adding_to_2020::<2>(&numbers)
        .expect("Couldn't determine a result for task 1.");

    println!("Task 1: {} * {} == {}", first, second, first * second);

    let [first, second, third] = find_entries_adding_to_2020::<3>(&numbers)
        .expect("Couldn't determine a result for task 2.");

    println!(
        "Task 2: {} * {} * {} == {}",
        first,
        second,
        third,
        first * second * third
    );
    Ok(())
}

fn find_entries_adding_to_2020<const SIZE: usize>(expenses: &[u32]) -> Option<[u32; SIZE]> {
    let result = expenses
        .iter()
        .permutations(SIZE)
        .find(|entries| entries.iter().fold(0, |a, &b| a + b) == 2020)?;

    result.into_iter().cloned().collect_vec().try_into().ok()
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
    fn check_two_numbers_adding_to_2020() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        let [first, second] = find_entries_adding_to_2020::<2>(&numbers).unwrap();

        assert_eq!(514579, first * second);
    }

    #[test]
    fn check_three_numbers_adding_to_2020() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        let [first, second, third] = find_entries_adding_to_2020::<3>(&numbers).unwrap();

        assert_eq!(241861950, first * second * third);
    }
}
