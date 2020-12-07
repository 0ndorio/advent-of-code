use std::collections::HashSet;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Error> {
    let groups = parse_input::<Group>()?;

    let num_yes_answers = groups
        .iter()
        .map(Group::generate_answer_set)
        .map(|answers| answers.len())
        .sum::<usize>();

    println!("Num Answers: {}", num_yes_answers);
    Ok(())
}

// ------------------------------------------------------------------------------
// Groups
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
struct Group(Vec<Form>);

impl Group {
    fn generate_answer_set(&self) -> HashSet<char> {
        self.0
            .iter()
            .flat_map(|form| form.0.iter())
            .cloned()
            .collect()
    }
}

impl FromStr for Group {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}

// ------------------------------------------------------------------------------
// Form
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
struct Form(HashSet<char>);

impl FromStr for Form {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let answers = s.chars().filter(|c| c.is_alphabetic()).collect();
        Ok(Self(answers))
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
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANSWER_LIST: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn calc_total_num_yes() -> Result<(), Error> {
        let num_total_answers = ANSWER_LIST
            .split("\n\n")
            .filter_map(|entry| entry.parse::<Group>().ok())
            .map(|group| group.generate_answer_set())
            .map(|answers| answers.len())
            .sum::<usize>();

        assert_eq!(11, num_total_answers);
        Ok(())
    }
}
