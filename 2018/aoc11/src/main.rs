use std::error::Error;

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

#[derive(Debug)]
struct FuelCell {
    rack_id: u32,
    power_level: i64,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<FuelCell>>,
    size: (usize, usize),
}

fn main() -> Result<()> {
    let serial_number = 7139;
    let grid = Grid::new(serial_number, 300, 300);

    let (coordinate, combined_power) = grid.find_max_power(3, 3)?;
    println!(
        "The 3x3 window with the max power level of {} is placed at {}x{}.",
        combined_power, coordinate.0, coordinate.1
    );

    let (coordinate, size, combined_power) = grid.find_max_power_window()?;
    println!(
        "The {}x{} window with the max power level of {} is placed at {}x{}.",
        size, size, combined_power, coordinate.0, coordinate.1
    );

    Ok(())
}

impl Grid {
    fn new(serial_number: u32, width: usize, height: usize) -> Self {
        let mut cells = vec![];

        for y in 0..height {
            let mut row = vec![];

            for x in 0..width {
                let rack_id: u32 = (x as u32) + 10;

                let mut power_level = i64::from(rack_id * (y as u32));
                power_level += i64::from(serial_number);
                power_level *= i64::from(rack_id);
                power_level /= 100;
                power_level %= 10;
                power_level -= 5;

                let cell = FuelCell {
                    rack_id,
                    power_level,
                };
                row.push(cell);
            }

            cells.push(row);
        }

        let size = (width, height);
        Grid { cells, size }
    }

    fn find_max_power(
        &self,
        window_width: usize,
        window_height: usize,
    ) -> Result<((usize, usize), i64)> {
        let max_x = self.size.0 - window_width;
        let max_y = self.size.1 - window_height;

        (0..max_y)
            .flat_map(|y| (0..max_x).map(move |x| (x, y)))
            .map(|(x, y)| {
                let combined_power = self
                    .cells
                    .iter()
                    .skip(y)
                    .take(window_height)
                    .map(|cells| {
                        cells
                            .iter()
                            .skip(x)
                            .take(window_width)
                            .map(|cell| cell.power_level)
                            .sum::<i64>()
                    })
                    .sum::<i64>();

                ((x, y), combined_power)
            })
            .max_by_key(|(_, power)| *power)
            .ok_or_else(|| Box::from("An empty grid has now max power window of any size."))
    }

    fn find_max_power_window(&self) -> Result<((usize, usize), usize, i64)> {
        let max_square_side = usize::min(self.size.0, self.size.1);

        (0..max_square_side)
            .flat_map(|size| {
                self.find_max_power(size, size)
                    .map(|(coordinate, power)| (coordinate, size, power))
                    .ok()
            })
            .max_by_key(|(_, _, power)| *power)
            .ok_or_else(|| Box::from("An empty grid has now max power window of any size."))
    }
}
