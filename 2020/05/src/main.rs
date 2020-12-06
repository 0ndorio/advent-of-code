use std::{env, fs};

use parse_display::{Display, FromStr};

fn main() -> Result<(), Error> {
    let boarding_passes = parse_input::<BoardingPass>()?;

    let max_seat_id = boarding_passes
        .iter()
        .map(BoardingPass::calc_seat_id)
        .max()
        .unwrap_or(0);

    println!("Max Set Id: {}", max_seat_id);

    Ok(())
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

type RowCount = u32;
type SeatCount = u32;

const PLANE_SIZE: (RowCount, SeatCount) = (128, 8);

#[derive(Debug, Display, FromStr)]
#[display("{row}{seat}")]
struct BoardingPass {
    #[from_str(regex = "[BF]+")]
    row: String,
    #[from_str(regex = "[LR]+")]
    seat: String,
}

impl BoardingPass {
    fn calc_seat_id(&self) -> u32 {
        let (row, seat) = self.calc_seat_position();
        row * 8 + seat
    }

    fn calc_seat_position(&self) -> (RowCount, SeatCount) {
        (self.calc_row(), self.calc_seat())
    }

    fn calc_row(&self) -> RowCount {
        let mut row = 0..PLANE_SIZE.0;
        let mut step_size = PLANE_SIZE.0;

        for hint in self.row.chars() {
            step_size /= 2;

            if 'B' == hint {
                row.start += step_size;
            } else {
                row.end -= step_size;
            }
        }

        row.start
    }

    fn calc_seat(&self) -> SeatCount {
        let mut seat = 0..PLANE_SIZE.1;
        let mut step_size = PLANE_SIZE.1;

        for hint in self.seat.chars() {
            step_size /= 2;

            if 'R' == hint {
                seat.start += step_size;
            } else {
                seat.end -= step_size;
            }
        }

        seat.start
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

    #[test]
    fn calc_score_example_boarding_passes() -> Result<(), Error> {
        let examples: Vec<(&str, (RowCount, SeatCount), u32)> = vec![
            ("BFFFBBFRRR", (70, 7), 567),
            ("FFFBBBFRRR", (14, 7), 119),
            ("BBFFBBFRLL", (102, 4), 820),
        ];

        for example in examples {
            let pass = example.0.parse::<BoardingPass>()?;
            assert_eq!(example.1 .0, pass.calc_row());
            assert_eq!(example.1 .1, pass.calc_seat());
            assert_eq!(example.2, pass.calc_seat_id());
        }

        Ok(())
    }
}
