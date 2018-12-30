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
    let step_order = determine_synchronous_step_order(&steps);

    let step_order = step_order.into_iter().collect::<String>();
    println!("Step order: {}", step_order);

    let steps = setup_steps(&input)?;
    let (step_order, duration) = calc_async_duration(&steps, 5, 60);

    let step_order = step_order.into_iter().collect::<String>();
    println!("Async step order: {} in {} seconds", step_order, duration);

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

fn determine_synchronous_step_order(steps: &HashMap<char, StepCell>) -> Vec<char> {
    let mut steps = steps.values().map(StepCell::clone).collect::<Vec<_>>();
    let mut final_order = vec![];

    while !steps.is_empty() {
        sort_remaining_steps(&mut steps);

        if let Some(next_step) = steps.pop() {
            {
                let parent_ref = next_step.borrow();
                for child in &parent_ref.parent_for {
                    let mut child_ref = child.borrow_mut();
                    child_ref.depends_on.remove(&next_step);
                }
            }

            final_order.push(next_step.borrow().identifier);
        }
    }

    final_order
}

fn sort_remaining_steps(steps: &mut [StepCell]) {
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
}

fn calc_async_duration(
    steps: &HashMap<char, StepCell>,
    num_worker: usize,
    base_step_duration: u32,
) -> (Vec<char>, u32) {
    let mut worker: Vec<Worker> = vec![Worker::default(); num_worker];
    let mut steps = steps.values().map(StepCell::clone).collect::<Vec<_>>();

    let mut final_order = vec![];
    let mut current_time = 0u32;

    sort_remaining_steps(&mut steps);

    loop {
        let finished = steps.is_empty() && worker.iter().all(|worker| worker.is_idle(current_time));
        if finished {
            break;
        }

        worker
            .iter_mut()
            .filter(|worker| worker.is_idle(current_time))
            .for_each(|worker| {
                let finished_step = std::mem::replace(&mut worker.step, None);
                if let Some(finished_step) = finished_step {

                    let parent_ref = finished_step.borrow();
                    for child in &parent_ref.parent_for {
                        let mut child_ref = child.borrow_mut();
                        child_ref.depends_on.remove(&finished_step);
                    }

                    let identifier = finished_step.borrow().identifier;
                    final_order.push(identifier);

                    sort_remaining_steps(&mut steps);
                }

                let step_available = if let Some(next_step) = steps.last() {
                    next_step.borrow().depends_on.is_empty()
                } else {
                    false
                };

                if step_available {
                    if let Some(next_step) = steps.pop() {
                        // Every identifier applies some custom delay.
                        let identifier = next_step.borrow().identifier;
                        let finish_time =
                            current_time + base_step_duration + (identifier as u32 - '@' as u32);

                        worker.start_work(&next_step, finish_time);
                    }
                }
            });

        current_time += 1;
    }

    (final_order, current_time)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Default)]
struct Worker {
    done_at: u32,
    step: Option<StepCell>,
}

impl Worker {
    pub fn start_work(&mut self, step: &StepCell, done_at: u32) {
        self.done_at = done_at;
        self.step = Some(StepCell::clone(step));
    }

    pub fn is_idle(&self, current_time: u32) -> bool {
        self.done_at <= current_time
    }
}
