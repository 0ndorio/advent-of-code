use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::Error;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alphanumeric1, anychar, digit1, space1};
use nom::Finish;
use nom::IResult;

fn main() -> Result<(), Error> {
    let password_entries: Vec<PasswordEntry> = parse_input()?;

    let num_valid_passwords = password_entries
        .iter()
        .map(PasswordEntry::is_valid)
        .filter(|&value| value)
        .count();
    println!("Num Valid Passwords: {}", num_valid_passwords);

    Ok(())
}

// ------------------------------------------------------------------------------
// PasswordEntry
// ------------------------------------------------------------------------------

struct PasswordEntry {
    rule: PasswordRule,
    password: String,
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let count = self
            .password
            .as_str()
            .chars()
            .filter(|&c| c == self.rule.character)
            .count();

        self.rule.range.contains(&count)
    }
}

impl FromStr for PasswordEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_entry(input: &str) -> IResult<&str, (&str, &str)> {
            let (remainder, rule) = take_until(":")(input)?;
            let (remainder, _) = tag(": ")(remainder)?;
            let (remainder, password) = alphanumeric1(remainder)?;

            Ok((remainder, (rule, password)))
        }

        match parse_entry(s).finish() {
            Ok((_, (rule, password))) => {
                let rule = rule.parse()?;
                let password = password.to_string();

                Ok(PasswordEntry { rule, password })
            }
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            })?,
        }
    }
}

// ------------------------------------------------------------------------------
// PasswordRule
// ------------------------------------------------------------------------------

struct PasswordRule {
    range: RangeInclusive<usize>,
    character: char,
}

impl FromStr for PasswordRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_rule(input: &str) -> IResult<&str, (&str, &str, char)> {
            let (remainder, start) = digit1(input)?;
            let (remainder, _) = tag("-")(remainder)?;
            let (remainder, end) = digit1(remainder)?;

            let (remainder, _) = space1(remainder)?;
            let (remainder, character) = anychar(remainder)?;

            Ok((remainder, (start, end, character)))
        }

        match parse_rule(s).finish() {
            Ok((_, (start, end, character))) => {
                let start = start.parse::<usize>()?;
                let end = end.parse::<usize>()?;
                let character = character;

                Ok(PasswordRule {
                    range: RangeInclusive::new(start, end),
                    character,
                })
            }
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            })?,
        }
    }
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

fn parse_input<ResultT>() -> Result<Vec<ResultT>, Error>
where
    ResultT: FromStr<Err = Error>,
{
    let input_file = env::args().nth(1).expect("input file name missing");

    fs::read_to_string(input_file)?
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
}

// ------------------------------------------------------------------------------
// Tests
// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_passwords() -> Result<(), Error> {
        let entries = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        let entry = entries[0].parse::<PasswordEntry>()?;
        assert!(entry.is_valid());

        let entry = entries[1].parse::<PasswordEntry>()?;
        assert!(!entry.is_valid());

        let entry = entries[2].parse::<PasswordEntry>()?;
        assert!(entry.is_valid());

        Ok(())
    }
}
