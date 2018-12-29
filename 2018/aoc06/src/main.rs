use std::{
    collections::HashMap,
    error::Error,
    io::{self, Read},
    str::FromStr,
};

mod coordinate;
use crate::coordinate::{Coordinate, Distance, Grid};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;
type Identifier = usize;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let coordinates: Vec<Coordinate> = input
        .lines()
        .map(Coordinate::from_str)
        .collect::<Result<_>>()?;

    let distances = calc_distances(&coordinates);

    let max_area = find_max_finite_area(&distances)?;
    println!("Max finite area {} with {}", max_area.0, max_area.1);

    let area_size = find_area_with_distances_under(&distances, 10000);
    println!("Area size: {}", area_size);

    Ok(())
}

fn calc_distances(spots: &[Coordinate]) -> Grid {
    let width = spots
        .iter()
        .map(|coordinate| coordinate.x)
        .max()
        .expect("We always operate on at least one coordinate");

    let height = spots
        .iter()
        .map(|coordinate| coordinate.y)
        .max()
        .expect("We always operate on at least one coordinate");

    let mut distance_overview = HashMap::new();
    for x in 0..=width {
        for y in 0..=height {
            let border_tile = x == 0 || y == 0 || x == width || y == height;
            let tile = Coordinate { x, y, border_tile };

            let owner: HashMap<Identifier, Distance> = spots
                .iter()
                .map(|coordinate| coordinate.distance(&tile))
                .enumerate()
                .collect();

            distance_overview.insert(tile, owner);
        }
    }

    distance_overview
}

fn find_max_finite_area(grid: &Grid) -> Result<(Identifier, usize)> {
    let mut ownership = HashMap::new();

    grid.iter()
        .map(|(coordinate, distances)| {
            let potential_owner = coordinate::find_owner(&distances);
            (coordinate, potential_owner)
        })
        .for_each(|(coordinate, potential_owner)| match potential_owner {
            None => (),
            Some(identifier) => {
                let coordinates = ownership.entry(identifier).or_insert_with(Vec::new);
                coordinates.push(coordinate);
            }
        });

    let (identifier, coordinates) = ownership
        .into_iter()
        .filter(|(_, coordinates)| coordinates.iter().all(|coordinate| !coordinate.border_tile))
        .max_by_key(|(_, coordinates)| coordinates.len())
        .ok_or_else(|| "Couldn't determine the size of any finite area.".to_string())?;

    Ok((identifier, coordinates.len()))
}

fn find_area_with_distances_under(grid: &Grid, upper_limit: u32) -> usize {
    grid.values()
        .map(coordinate::total_distance)
        .filter(|&distance| distance < upper_limit)
        .count()
}
