use std::{
    collections::HashMap,
    error::Error,
    io::{self, Read},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

pub type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let claims: Vec<Claim> = input.split('\n').map(Claim::from_str).flatten().collect();
    let fabric_plan = generate_fabric_plan(&claims)?;

    let num_squares = calc_overlapping_squares(&fabric_plan)?;
    println!("num squares: {}", num_squares);

    Ok(())
}


fn generate_fabric_plan(claims: &[Claim]) -> Result<HashMap<(u32, u32), u32>> {
    let mut fabric_plan = HashMap::<(u32, u32), u32>::new();

    for claim in claims {
        let (x, y) = claim.origin;
        let (width, height) = claim.size;

        for x_index in x..(x + width) {
            for y_index in y..(y + height) {
                let counter = fabric_plan.entry((x_index, y_index)).or_insert(0);
                *counter += 1;
            }
        }
    }

    Ok(fabric_plan)
}

fn calc_overlapping_squares(fabric_plan: &HashMap<(u32, u32), u32>) -> Result<usize> {
    let overlapping = fabric_plan.values().filter(|&&counter| counter > 1).count();
    Ok(overlapping)
}

// -----------------------------------------------------------------------------
// Claim
// -----------------------------------------------------------------------------

pub struct Claim {
    _id: u32,
    origin: (u32, u32),
    size: (u32, u32),
}

impl FromStr for Claim {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        lazy_static! {
            static ref CLAIM_RE: Regex = Regex::new(
                r"^#(?P<id>\d+) @ (?P<origin_x>\d+),(?P<origin_y>\d+): (?P<width>\d+)x(?P<height>\d+)",
            ).expect("Predefined claim regex failed to compile.");
        }

        let capture = CLAIM_RE
            .captures(input)
            .ok_or_else(|| format!("Couldn't parse claim line: {}", input))?;

        let _id = capture["id"].parse()?;
        let origin = (capture["origin_x"].parse()?, capture["origin_y"].parse()?);
        let size = (capture["width"].parse()?, capture["height"].parse()?);

        Ok(Claim { _id, origin, size })
    }
}
