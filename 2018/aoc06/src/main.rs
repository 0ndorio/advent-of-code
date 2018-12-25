use std::{
    collections::HashMap,
    error::Error,
    io::{self, Read},
    str::FromStr,
};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

struct Coordinate {
    x: u32,
    y: u32,
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let coordinates: Vec<Coordinate> = input
        .lines()
        .map(Coordinate::from_str)
        .collect::<Result<Vec<_>>>()?;

    let ownership = calc_coordinate_ownership(&coordinates);

    let max_area = ownership
        .iter()
        .filter(|(_, (_, owns_border))| !*owns_border)
        .max_by_key(|(_, (owned_tiles, _))| *owned_tiles)
        .ok_or("Couldn't determine any finite area.")?;

    println!("Max finite area {} with {}", max_area.0, (max_area.1).0);

    Ok(())
}

fn calc_coordinate_ownership(coordinates: &[Coordinate]) -> HashMap<usize, (u32, bool)> {
    let width = coordinates
        .iter()
        .map(|coordinate| coordinate.x)
        .max()
        .expect("We always operate on at least one coordinate");

    let height = coordinates
        .iter()
        .map(|coordinate| coordinate.y)
        .max()
        .expect("We always operate on at least one coordinate");

    let mut ownership: HashMap<usize, (u32, bool)> = HashMap::new();

    for x in 0..=width {
        for y in 0..=height {
            let border_tile = x == 0 || y == 0 || x == width || y == height;

            let current = Coordinate { x, y };
            let owner = coordinates
                .iter()
                .map(|coordinate| coordinate.distance(&current))
                .enumerate()
                .fold(vec![], |mut min, next| {
                    match min.first() {
                        None => min.push(next),
                        Some(old) => {
                            if next.1 == old.1 {
                                min.push(next);
                            } else if next.1 < old.1 {
                                min.clear();
                                min.push(next);
                            }
                        }
                    }

                    min
                });

            if owner.len() == 1 {
                if let Some((index, _)) = owner.first() {
                    let territory = ownership.entry(*index).or_insert((0u32, false));
                    territory.0 += 1u32;
                    territory.1 |= border_tile;
                }
            }
        }
    }

    ownership
}

impl Coordinate {
    fn distance(&self, target: &Coordinate) -> u32 {
        let x_distance = self.x.max(target.x) - self.x.min(target.x);
        let y_distance = self.y.max(target.y) - self.y.min(target.y);

        x_distance + y_distance
    }
}

impl FromStr for Coordinate {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let mut values = input.split(',').map(str::trim).flat_map(str::parse::<u32>);

        let x = values
            .next()
            .ok_or_else(|| "Couldn't parse x-value.".to_string())?;
        let y = values
            .next()
            .ok_or_else(|| "Couldn't parse y-value.".to_string())?;

        Ok(Self { x, y })
    }
}
