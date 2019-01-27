use std::{
    collections::{BTreeMap, HashMap, HashSet},
    error::Error,
    io::{self, Read},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

use aoc16::{instruction::Instruction, operation::Executor, state::State, Result};

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

    let real_opcodes = find_real_opcodes(&examples);
    println!("Current opcodes are mapped like: {:#?}", real_opcodes);

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

fn find_real_opcodes(examples: &[Example]) -> BTreeMap<u8, u8> {
    let mut opcode_candidates = HashMap::new();

    for example in examples {
        let original_opcode = example.instruction.opcode;
        for opcode in 0..=15 {
            let mut instruction = example.instruction.clone();
            instruction.opcode = opcode;

            let state = Executor::default()
                .with_state(example.before.clone())
                .with_instruction(instruction)
                .run()
                .clone();

            if state == example.after {
                let candidate_list = opcode_candidates
                    .entry(original_opcode)
                    .or_insert_with(HashSet::new);
                candidate_list.insert(opcode);
            }
        }
    }

    let mut candidate_lists: Vec<(u8, HashSet<u8>)> = opcode_candidates.into_iter().collect();

    let mut mapping = BTreeMap::new();
    loop {
        candidate_lists.sort_by_key(|(_, candidates)| candidates.len());
        candidate_lists.reverse();

        let (opcode, candidates) = match candidate_lists.pop() {
            None => break,
            Some(entry) => entry,
        };

        let candidates: Vec<u8> = candidates.into_iter().collect();
        let real_opcode = candidates[0];

        candidate_lists.iter_mut().for_each(|(_, candidates)| {
            candidates.remove(&real_opcode);
        });
        mapping.insert(opcode, candidates[0]);
    }

    mapping
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
