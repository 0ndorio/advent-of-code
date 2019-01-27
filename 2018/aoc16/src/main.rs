mod instruction;
mod operation;
mod state;

use std::{
    error::Error,
    io::{self, Read},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{instruction::Instruction, operation::Executor, state::State};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let examples: Result<Vec<Example>> = input.split("\n\n").map(Example::from_str).collect();
    let examples = examples?;

    let behaves_like_3_or_more = find_overlapping_examples(&examples);
    println!(
        "Instructions that behave like 3 or more opcodes: {}",
        behaves_like_3_or_more
    );

    Ok(())
}

fn find_overlapping_examples(examples: &[Example]) -> u32 {
    let mut behaves_like_3_or_more = 0;

    for example in examples {
        let mut num_potential_operations = 0;

        for opcode in 0..=15 {
            let mut instruction = example.instruction.clone();
            instruction.opcode = opcode;

            let state = Executor::default()
                .with_state(example.before.clone())
                .with_instruction(instruction)
                .run()
                .clone();

            if state == example.after {
                num_potential_operations += 1;
            }
        }

        if num_potential_operations >= 3 {
            behaves_like_3_or_more += 1;
        }
    }

    behaves_like_3_or_more
}

#[derive(Debug)]
struct Example {
    after: State,
    before: State,
    instruction: Instruction,
}

impl FromStr for Example {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        lazy_static! {
            static ref EXAMPLE_RE: Regex = Regex::new(
                r"Before: (?P<state_before>.*)\s(?P<instruction>.*)\sAfter: (?P<state_after>.*)",
            )
            .expect("Predefined example regex failed to compile.");
        };

        let captures = EXAMPLE_RE
            .captures(&input)
            .ok_or_else(|| format!("Couldn't parse input entry line: {}", input))?;

        let before = captures["state_before"].parse()?;
        let after = captures["state_after"].parse()?;
        let instruction = captures["instruction"].parse()?;

        Ok(Self {
            after,
            before,
            instruction,
        })
    }
}
