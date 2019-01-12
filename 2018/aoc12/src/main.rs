use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Display, Formatter},
    io::{self, Read},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let garden = Garden::from_str(&input)?;
    garden.clone().calc_generation_sum(20);
    garden.clone().calc_generation_sum(50);
    garden.clone().calc_generation_sum(500);
    garden.clone().calc_generation_sum(5000);
    garden.clone().calc_generation_sum(50000);

    // As this solution is way to slow to calculate the expected
    // 50 billion generations it tries to generate the minimal
    // possible amount.
    //
    // To achieve this goal it trims the empty pots on both sides
    // and stores the transition from one generation into the next
    // until it encounters an already known generation.
    //
    // In this case it checks if it calculated enough generations
    // to discover a cycle inside the stored transitions. If such
    // a cycle exist it reduces the number of required generations
    // to the reminder of the division between the missing generations
    // and the cycle size.
    //
    // This solution has a major flaw. It can't determine the actual
    // plant ids and therefore the right amount of points a generation
    // is worth. We are still able to determine a proper result as the
    // the discovered cycle is actually a shift to the right so that
    // the final result only needs to add the amount of skipped
    // generations to every living plant id.
    garden.clone().calc_generation_sum(50_000_000_000);

    Ok(())
}

#[derive(Debug, Clone)]
struct Garden {
    generation: u32,

    pots: HashMap<i32, char>,
    rules: HashMap<u32, char>,

    context: String,
    environments: HashMap<String, (HashMap<i32, char>, String, i32, i32)>,

    smallest_plant_id: i32,
    greatest_plant_id: i32,
}

#[derive(Debug)]
struct Rule {
    pots: String,
}

impl Garden {
    fn calc_generation_sum(&mut self, num_generations: u64) {
        let skipped_generations = self.fast_forward(num_generations) as i64;

        let sum: i64 = self.pots
            .iter()
            .filter(|(_, symbol)| **symbol == '#')
            .map(|(index, _)| i64::from(*index) + skipped_generations)
            .sum();

        println!("Garden value after generation {}: {}", num_generations, sum);
    }

    fn fast_forward(&mut self, num_generations: u64) -> u64 {
        let cycle_size = |garden: &Garden| {
            let mut cycle = HashSet::new();
            let mut cycle_size = 0;

            let mut context = garden.context.as_str();

            loop {
                if let Some(environment) = garden.environments.get(context) {
                    context = environment.1.as_str();
                    if cycle.insert(context) {
                        cycle_size += 1;
                    } else {
                        break Some(cycle_size);
                    }
                } else {
                    break None;
                }
            }
        };

        let mut num_generations = num_generations;
        let skipped_generations = loop {
            if num_generations == 0 {
                break 0;
            }

            let known_environment = self.environments.contains_key(&self.context);
            if !known_environment {
                self.next_generation();
                num_generations -= 1;
                continue;
            }

            if let Some(cycle_size) = cycle_size(&self) {
                let skipped_generations = num_generations;
                num_generations %= cycle_size;
                break skipped_generations;
            }
        };

        for _ in 0..num_generations {
            self.next_generation();
        }

        skipped_generations
    }

    fn next_generation(&mut self) {
        static RULE_LENGTH: i32 = 5;
        static HALF_RULE_LENGTH: i32 = 2;

        self.generation += 1;

        let smallest_id = &mut self.smallest_plant_id;
        let greatest_id = &mut self.greatest_plant_id;

        let rules = &self.rules;

        let mut new_generation = self.pots.clone();
        let old_generation = &mut self.pots;

        let from = *smallest_id - RULE_LENGTH;
        let to = *greatest_id + RULE_LENGTH;

        let mut new_context = String::new();
        for index in from..=to {
            let from = index - HALF_RULE_LENGTH;
            let to = index + HALF_RULE_LENGTH;

            let hash = (from..=to).fold(0, |acc, index| {
                let actual_symbol = old_generation.get(&index).unwrap_or(&'.');
                match actual_symbol {
                    '#' => (acc << 2) + 1,
                    _ => (acc << 2),
                }
            });

            let symbol = new_generation.entry(index).or_insert('.');

            let applies = rules.contains_key(&hash);
            let new_symbol = if applies {
                if index < *smallest_id {
                    *smallest_id = index;
                } else if index > *greatest_id {
                    *greatest_id = index;
                }

                '#'
            } else {
                '.'
            };

            *symbol = new_symbol;
            new_context.push(new_symbol);
        }

        let new_context = new_context.trim_matches('.').to_string();
        let environment = (
            new_generation.clone(),
            new_context.clone(),
            *smallest_id,
            *greatest_id,
        );

        self.environments.insert(self.context.clone(), environment);
        self.context = new_context;
        self.pots = new_generation;
    }
}

impl FromStr for Garden {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let generation = 0;

        lazy_static! {
            static ref STATE_RE: Regex =
                Regex::new(r"initial state: (?P<pots>[#.]*)\s*(?P<rules>([#.]+ => [#.]\s)*)")
                    .expect("Predefined initial state regex failed to compile.");
        }

        let capture = STATE_RE.captures(input).ok_or_else(|| {
            format!(
                "Couldn't parse the garden state or rules from input: {}",
                input
            )
        })?;

        let pots = capture["pots"]
            .chars()
            .enumerate()
            .map(|(index, symbol)| (index as i32, symbol))
            .collect::<HashMap<_, _>>();

        let rules = capture["rules"]
            .lines()
            .flat_map(Rule::from_str)
            .map(|rule| {
                let hash = rule.pots.chars().fold(0, |acc, symbol| match symbol {
                    '#' => (acc << 2) + 1,
                    _ => (acc << 2),
                });

                (hash, '#')
            })
            .collect::<HashMap<_, _>>();

        let smallest_plant_id = 0;
        let greatest_plant_id = pots.len() as i32;

        let context = capture["pots"].trim_matches('.').to_string();
        let environments = HashMap::new();

        Ok(Self {
            generation,
            pots,
            rules,
            context,
            environments,
            smallest_plant_id,
            greatest_plant_id,
        })
    }
}

impl Display for Garden {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut pots = self.pots.iter().collect::<Vec<_>>();
        pots.sort_by_key(|(index, _)| **index);

        let pots = pots.iter().map(|(_, symbol)| **symbol).collect::<String>();
        write!(f, "{:10}: {} [{}]", self.generation, pots, pots.len())
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(r"(?P<pots>[#.]+) => #",)
                .expect("Predefined rule regex failed to compile.");
        }

        let capture = RULE_RE
            .captures(input)
            .ok_or_else(|| format!("Couldn't parse the rule from line: {}", input))?;

        let pots = String::from(&capture["pots"]);
        Ok(Rule { pots })
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} => #", self.pots)
    }
}
