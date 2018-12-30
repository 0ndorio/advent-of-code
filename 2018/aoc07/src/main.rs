use std::{
    cmp::Ordering,
    collections::HashMap,
    error::Error,
    io::{self, Read},
};

use lazy_static::lazy_static;
use regex::Regex;

mod step;
use crate::step::{Step, StepCell};

pub type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let steps = setup_steps(&input)?;

    let step_order = determine_step_order(&steps);
    let step_order = step_order.into_iter().collect::<String>();
    println!("Step order: {}", step_order);

    Ok(())
}

fn setup_steps(input: &str) -> Result<HashMap<char, StepCell>> {
    let mut steps = HashMap::new();

    for line in input.lines() {
        lazy_static! {
            static ref STEP_DEPENDENCY_RE: Regex =
                Regex::new(r"^Step (?P<parent>\w) .+ step (?P<child>\w) .+",)
                    .expect("Predefined log entry regex failed to compile.");
        }

        let captures = STEP_DEPENDENCY_RE
            .captures(line)
            .ok_or_else(|| format!("Couldn't parse input entry line: {}", line))?;

        let parent = {
            let parent: char = captures["parent"].parse()?;
            let entry = steps
                .entry(parent)
                .or_insert_with(|| StepCell::new(Step::new(parent)));

            StepCell::clone(entry)
        };

        let child = {
            let child: char = captures["child"].parse()?;
            let entry = steps
                .entry(child)
                .or_insert_with(|| StepCell::new(Step::new(child)));

            StepCell::clone(entry)
        };

        {
            let child = StepCell::clone(&child);

            let mut parent_ref = parent.borrow_mut();
            parent_ref.parent_for.insert(child);
        }

        {
            let parent = StepCell::clone(&parent);

            let mut child_ref = child.borrow_mut();
            child_ref.depends_on.insert(parent);
        }
    }

    Ok(steps)
}

fn determine_step_order(steps: &HashMap<char, StepCell>) -> Vec<char> {
    let mut steps = steps.iter().map(|(_, step)| step).collect::<Vec<_>>();
    let mut final_order = vec![];

    while !steps.is_empty() {
        steps.sort_by(|lhs, rhs| {
            let lhs = lhs.borrow();
            let rhs = rhs.borrow();

            let lhs_len = lhs.depends_on.len();
            let rhs_len = rhs.depends_on.len();

            let ordering = rhs_len.cmp(&lhs_len);
            match ordering {
                Ordering::Equal => rhs.cmp(&lhs),
                _ => ordering,
            }
        });

        if let Some(next_step) = steps.pop() {
            let parent_ref = next_step.borrow();
            for child in &parent_ref.parent_for {
                let mut child_ref = child.borrow_mut();
                child_ref.depends_on.remove(&next_step);
            }

            final_order.push(parent_ref.identifier);
        }
    }

    final_order
}
