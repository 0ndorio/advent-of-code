use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use std::{env, fs};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let rows = parse_input::<Row>()?;
    let map = Map(rows);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let num_trees = slopes
        .iter()
        .map(|slope| map.count_trees(slope))
        .product::<u128>();

    println!("Total Number of Trees: {}", num_trees);

    Ok(())
}

// ------------------------------------------------------------------------------
// Map
// ------------------------------------------------------------------------------

#[derive(Debug)]
struct Map(pub Vec<Row>);

impl Map {
    fn count_trees(&self, slope: &(usize, usize)) -> u128 {
        let target_depth = self.0.len();

        let mut num_trees = 0;
        let mut position = (0, 0);

        while position.1 < target_depth {
            let row = &self.0[position.1].0;
            let row_length = row.len();

            if row[position.0] == Tile::Tree {
                num_trees += 1;
            }

            position.0 = (position.0 + slope.0) % row_length;
            position.1 += slope.1;
        }

        num_trees
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.0 {
            row.fmt(f)?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Map(rows))
    }
}

// ------------------------------------------------------------------------------
// Row
// ------------------------------------------------------------------------------

#[derive(Debug)]
struct Row(pub Vec<Tile>);

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.iter().map(|tile| tile.fmt(f)).collect()
    }
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.trim().split("");
        let tiles = chars
            .into_iter()
            .map(FromStr::from_str)
            .filter(Result::is_ok)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Row(tiles))
    }
}

// ------------------------------------------------------------------------------
// Tile
// ------------------------------------------------------------------------------

#[derive(Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Tree,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::Tree => f.write_char('#'),
        }
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Tree),
            _ => Err("Unknown Tile".into()),
        }
    }
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

fn parse_input<ResultT>() -> Result<Vec<ResultT>, Error>
where
    ResultT: std::str::FromStr<Err = Error>,
{
    let input_file = env::args().nth(1).expect("input file name missing");

    fs::read_to_string(input_file)?
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {

    use super::*;

    const MAP: &str = r#"..##.......
                          #...#...#..
                          .#....#..#.
                          ..#.#...#.#
                          .#...##..#.
                          ..#.##.....
                          .#.#.#....#
                          .#........#
                          #.##...#...
                          #...##....#
                          .#..#...#.#"#;

    #[test]
    fn count_trees_in_3_1_slope() -> Result<(), Error> {
        let map = Map::from_str(MAP)?;
        assert_eq!(7, map.count_trees(&(3, 1)));

        Ok(())
    }

    #[test]
    fn count_trees_in_1_1_slope() -> Result<(), Error> {
        let map = Map::from_str(MAP)?;
        assert_eq!(2, map.count_trees(&(1, 1)));

        Ok(())
    }

    #[test]
    fn count_trees_in_5_1_slope() -> Result<(), Error> {
        let map = Map::from_str(MAP)?;
        assert_eq!(3, map.count_trees(&(6, 1)));

        Ok(())
    }

    #[test]
    fn count_trees_in_7_1_slope() -> Result<(), Error> {
        let map = Map::from_str(MAP)?;
        assert_eq!(4, map.count_trees(&(7, 1)));

        Ok(())
    }

    #[test]
    fn count_trees_in_1_2_slope() -> Result<(), Error> {
        let map = Map::from_str(MAP)?;
        assert_eq!(2, map.count_trees(&(1, 2)));

        Ok(())
    }
}
