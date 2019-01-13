use std::{
    collections::BTreeMap,
    error::Error,
    fmt::{self, Display, Formatter},
    io::{self, Read},
    mem,
    str::FromStr,
};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut map = Map::from_str(&input)?;
    run(&mut map)?;

    Ok(())
}

fn run(map: &mut Map) -> Result<()> {
    loop {
        let crashes = map.step()?;
        for (y, x) in crashes {
            println!("Discovered a crash at coordinate {}x{}.", x, y);
        }

        let num_carts = map.carts.len();
        if num_carts == 1 {
            if let Some((y, x)) = map.carts.keys().last() {
                println!("Coordinate of the last cart: {}x{}.", x, y);
            }

            break Ok(());
        } else if num_carts == 0 {
            println!("No carts left after the last crash.");

            break Ok(());
        }
    }
}

type Location = (usize, usize);

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    carts: BTreeMap<Location, Cart>,
}

#[derive(Debug, Clone)]
struct Cart {
    direction: Direction,
    intersection_count: u8,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    North,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
enum Rail {
    Curve(CurveType),
    Intersection,
    Straight,
}

#[derive(Debug, Copy, Clone)]
enum CurveType {
    Falling,
    Rising,
}

impl Map {
    /// Run a single time step. Return a crash location if one exist.
    fn step(&mut self) -> Result<Vec<(usize, usize)>> {
        let mut crashes = vec![];

        let mut old_state = mem::replace(&mut self.carts, BTreeMap::new());
        let mut carts: Vec<((usize, usize), Cart)> = old_state.clone().into_iter().collect();
        carts.reverse();

        while let Some(((y, x), mut cart)) = carts.pop() {
            if crashes.contains(&(y, x)) {
                continue;
            }

            let coordinate = match cart.direction {
                Direction::East => (y, x + 1),
                Direction::North => (y - 1, x),
                Direction::South => (y + 1, x),
                Direction::West => (y, x - 1),
            };

            let rail = Rail::from_char(self.map[coordinate.0][coordinate.1])?;
            cart.update(rail);

            // A crash could happen with a cart that still waits for its move or an already
            // moved one.
            if old_state.contains_key(&coordinate) || self.carts.contains_key(&coordinate) {
                self.carts.remove(&coordinate);
                old_state.remove(&coordinate);

                crashes.push(coordinate);
            } else {
                self.carts.insert(coordinate, cart);
            }

            old_state.remove(&(y, x));
        }

        Ok(crashes)
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let mut map: Vec<Vec<char>> = input
            .lines()
            .map(str::chars)
            .map(Iterator::collect)
            .collect();

        let mut carts = BTreeMap::new();
        for (y, row) in map.iter().enumerate() {
            for (x, symbol) in row.iter().enumerate() {
                if let Ok(direction) = Direction::from_char(*symbol) {
                    let cart = Cart::new(direction);
                    carts.insert((y, x), cart);
                }
            }
        }

        carts.iter().for_each(|((y, x), cart)| {
            map[*y][*x] = match cart.direction {
                Direction::East | Direction::West => '-',
                Direction::North | Direction::South => '|',
            };
        });

        Ok(Map { map, carts })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut map = self.map.clone();
        for ((y, x), cart) in &self.carts {
            let symbol = match cart.direction {
                Direction::East => '>',
                Direction::North => '^',
                Direction::South => 'v',
                Direction::West => '<',
            };

            map[*y][*x] = symbol;
        }

        for line in map {
            let line: String = line.iter().collect();
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl Direction {
    fn turn_clockwise(self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_counter_clockwise(self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

impl Direction {
    fn from_char(symbol: char) -> Result<Self> {
        match symbol {
            '>' => Ok(Direction::East),
            '^' => Ok(Direction::North),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(Box::from(format!(
                "{} does not match any direction",
                symbol
            ))),
        }
    }
}

impl Rail {
    fn from_char(symbol: char) -> Result<Self> {
        let rail = match symbol {
            '/' => Rail::Curve(CurveType::Rising),
            '\\' => Rail::Curve(CurveType::Falling),
            '|' | '-' => Rail::Straight,
            '+' => Rail::Intersection,
            _ => return Err(format!("{} does not match any rail type", symbol))?,
        };

        Ok(rail)
    }
}

impl Cart {
    fn new(direction: Direction) -> Self {
        Cart {
            direction,
            intersection_count: 0,
        }
    }

    fn update(&mut self, rail: Rail) {
        let direction = match rail {
            Rail::Intersection => self.handle_intersection(),
            Rail::Straight => self.handle_straight(),
            Rail::Curve(kind) => self.handle_curve(kind),
        };

        self.direction = direction;
    }

    fn handle_curve(&mut self, curve: CurveType) -> Direction {
        match (curve, self.direction) {
            (CurveType::Falling, Direction::East)
            | (CurveType::Falling, Direction::West)
            | (CurveType::Rising, Direction::North)
            | (CurveType::Rising, Direction::South) => self.direction.turn_clockwise(),

            (CurveType::Falling, Direction::North)
            | (CurveType::Falling, Direction::South)
            | (CurveType::Rising, Direction::East)
            | (CurveType::Rising, Direction::West) => self.direction.turn_counter_clockwise(),
        }
    }

    fn handle_intersection(&mut self) -> Direction {
        let direction = match &self.intersection_count {
            0 => self.direction.turn_counter_clockwise(),
            1 => self.direction,
            2 => self.direction.turn_clockwise(),
            _ => unreachable!(),
        };

        self.intersection_count += 1;
        self.intersection_count %= 3;

        direction
    }

    fn handle_straight(&mut self) -> Direction {
        self.direction
    }
}
