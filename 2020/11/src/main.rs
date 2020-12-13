use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Error> {
    let rows = parse_input::<Row>()?;
    let mut map = Map::new(rows);

    let num_occupied = map.advance_until_stall();
    println!("Max Num Occupied: {}", num_occupied);

    Ok(())
}

// ------------------------------------------------------------------------------
// Map
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
struct Map {
    map: Vec<Row>,
    size: (usize, usize),
}

impl Map {
    fn new(rows: Vec<Row>) -> Self {
        let size = (rows.len(), rows[0].0.len());
        Map { map: rows, size }
    }

    fn advance_until_stall(&mut self) -> usize {
        let mut num_occupied = self.num_occupied();

        loop {
            self.advance();

            let updated_num_occupied = self.num_occupied();
            if num_occupied == updated_num_occupied {
                break;
            }

            num_occupied = updated_num_occupied;
        }

        num_occupied
    }

    fn advance(&mut self) {
        let mut new_map = self.map.clone();

        for row in 0..self.map.len() {
            for column in 0..self.map[row].0.len() {
                let updated_tile = self.update_tile(row, column);
                new_map[row].0[column] = updated_tile;
            }
        }

        self.map = new_map;
    }

    fn num_occupied(&self) -> usize {
        self.map.iter().map(Row::num_occupied).sum()
    }

    fn get_tile(&self, row: usize, column: usize) -> Option<Tile> {
        let row = self.map.get(row)?;
        row.0.get(column).cloned()
    }

    fn trace_view(&self, position: (usize, usize), direction: (i32, i32)) -> Tile {
        if direction == (0, 0) {
            return self
                .get_tile(position.0, position.1)
                .expect("trace failed cause of wrong initial position");
        }

        let mut position = position;
        loop {
            let (update, out_of_map) = match direction.0 {
                row if row > 0 => position.0.overflowing_add(1),
                0 => position.0.overflowing_add(0),
                _ => position.0.overflowing_sub(1),
            };

            if out_of_map {
                return Tile::EmptySeat;
            } else {
                position.0 = update;
            }

            let (update, out_of_map) = match direction.1 {
                column if column > 0 => position.1.overflowing_add(1),
                0 => position.1.overflowing_add(0),
                _ => position.1.overflowing_sub(1),
            };

            if out_of_map {
                return Tile::EmptySeat;
            } else {
                position.1 = update;
            }

            match self.get_tile(position.0, position.1) {
                Some(Tile::Floor) => {}
                Some(seat) => return seat,
                None => return Tile::EmptySeat,
            }
        }
    }

    fn update_tile(&self, row: usize, column: usize) -> Tile {
        let tile = self.map[row].0[column];
        if tile.is_floor() {
            return tile;
        }

        let min_row = row.saturating_sub(1);
        let max_row = row.saturating_add(1);

        let min_column = column.saturating_sub(1);
        let max_column = column.saturating_add(1);

        let mut adjacent_tiles = vec![];
        for adjacent_row in min_row..=max_row {
            for adjacent_column in min_column..=max_column {
                let mut row_direction =
                    i32::try_from(adjacent_row).expect("num rows is way to large");
                row_direction -= i32::try_from(row).expect("num rows is way to large");

                let mut column_direction =
                    i32::try_from(adjacent_column).expect("num columns is way to large");
                column_direction -= i32::try_from(column).expect("num columns is way to large");

                let direction = (row_direction, column_direction);
                let tile = self.trace_view((row, column), direction);
                adjacent_tiles.push(tile);
            }
        }

        let num_occupied = adjacent_tiles
            .into_iter()
            .filter(|tile| tile.is_occupied_seat())
            .count();

        if tile.is_empty_seat() && num_occupied == 0 {
            return Tile::OccupiedSeat;
        }

        if tile.is_occupied_seat() && num_occupied >= 6 {
            return Tile::EmptySeat;
        }

        tile
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.map {
            row.fmt(f)?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------
// Row
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
struct Row(Vec<Tile>);

impl Row {
    fn num_occupied(&self) -> usize {
        self.0
            .iter()
            .filter(|&&tile| tile == Tile::OccupiedSeat)
            .count()
    }
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s.chars().map(Tile::try_from).collect::<Result<_, _>>()?;
        Ok(Self(tiles))
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for tile in &self.0 {
            tile.fmt(f)?;
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------
// Tile
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Tile {
    fn is_floor(self) -> bool {
        self == Tile::Floor
    }

    fn is_empty_seat(self) -> bool {
        self == Tile::EmptySeat
    }

    fn is_occupied_seat(self) -> bool {
        self == Tile::OccupiedSeat
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tile = match value {
            '.' => Self::Floor,
            'L' => Self::EmptySeat,
            '#' => Self::OccupiedSeat,
            value => return Err(format!("Unknown Tile Value: {}", value).into()),
        };

        Ok(tile)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let tile = match self {
            Tile::Floor => '.',
            Tile::EmptySeat => 'L',
            Tile::OccupiedSeat => '#',
        };

        tile.fmt(f)
    }
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

type Error = Box<dyn std::error::Error>;

fn parse_input<ResultT>() -> Result<Vec<ResultT>, Error>
where
    ResultT: std::str::FromStr,
    ResultT::Err: Into<Error>,
{
    let input_file = env::args().nth(1).expect("input file name missing");

    fs::read_to_string(input_file)?
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: &str = "L.LL.LL.LL
                       LLLLLLL.LL
                       L.L.L..L..
                       LLLL.LL.LL
                       L.LL.LL.LL
                       L.LLLLL.LL
                       ..L.L.....
                       LLLLLLLLLL
                       L.LLLLLL.L
                       L.LLLLL.LL";

    #[test]
    fn simulate_5_rounds() -> Result<(), Error> {
        let rows = MAP
            .lines()
            .map(str::trim)
            .map(Row::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let mut map = Map::new(rows);
        let num_occupied = map.advance_until_stall();

        assert_eq!(26, num_occupied);
        Ok(())
    }
}
