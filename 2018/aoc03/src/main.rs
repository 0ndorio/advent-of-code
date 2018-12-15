use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::{self, Read},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;
type FabricPlan = HashMap<(u32, u32), Vec<u32>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let claims: Vec<Claim> = input.split('\n').map(Claim::from_str).flatten().collect();
    let plan = generate_plan(&claims)?;

    let num_squares = calc_overlapping_squares(&plan)?;
    println!("num squares: {}", num_squares);

    let unique_claim_id = find_unique_claim(&plan, &claims)?;
    println!("id of the unique claim: {}", unique_claim_id);

    Ok(())
}

fn generate_plan(claims: &[Claim]) -> Result<FabricPlan> {
    let mut plan = FabricPlan::new();

    for claim in claims {
        let (x, y) = claim.origin;
        let (width, height) = claim.size;

        for x_index in x..(x + width) {
            for y_index in y..(y + height) {
                let ids = plan.entry((x_index, y_index)).or_insert_with(|| vec![]);
                ids.push(claim.id);
            }
        }
    }

    Ok(plan)
}

fn calc_overlapping_squares(plan: &FabricPlan) -> Result<usize> {
    let overlapping = plan.values().filter(|ids| ids.len() > 1).count();
    Ok(overlapping)
}

fn find_unique_claim(plan: &FabricPlan, claims: &[Claim]) -> Result<u32> {
    let overlapping_ids: HashSet<&u32> = plan
        .values()
        .filter(|ids| ids.len() > 1)
        .flatten()
        .collect();

    let unique_claim = claims
        .iter()
        .filter(|claim| !overlapping_ids.contains(&claim.id))
        .last()
        .ok_or_else(|| "Couldn't determine any unique claim.".to_string())?;

    Ok(unique_claim.id)
}

// -----------------------------------------------------------------------------
// Claim
// -----------------------------------------------------------------------------

pub struct Claim {
    id: u32,
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

        let id = capture["id"].parse()?;
        let origin = (capture["origin_x"].parse()?, capture["origin_y"].parse()?);
        let size = (capture["width"].parse()?, capture["height"].parse()?);

        Ok(Claim { id, origin, size })
    }
}
