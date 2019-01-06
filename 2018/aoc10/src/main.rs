use std::{
    error::Error,
    io::{self, Read},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
struct Light {
    coordinate: Vec2,
    velocity: Vec2,
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lights = input.lines().flat_map(Light::from_str).collect::<Vec<_>>();

    let seconds = discover_message(&mut lights);
    print_message(&lights);

    println!("\nDiscovered message after {} seconds.", seconds);

    Ok(())
}

fn discover_message(lights: &mut [Light]) -> u32 {
    let mut min_area = u64::max_value();
    let mut counter = 0;

    loop {
        let min = min_boundary(&lights);
        let max = max_boundary(&lights);

        let width = (max.x - min.x).abs() as u64;
        let height = (max.y - min.y).abs() as u64;
        let new_area = height * width;

        if new_area > min_area {
            counter -= 1;
            lights.iter_mut().for_each(Light::step_back);

            break;
        }

        min_area = new_area;
        counter += 1;

        lights.iter_mut().for_each(Light::update)
    }

    counter
}

fn min_boundary(lights: &[Light]) -> Vec2 {
    let min_x = lights
        .iter()
        .map(|light| light.coordinate.x)
        .min()
        .unwrap_or_else(i32::min_value);

    let min_y = lights
        .iter()
        .map(|light| light.coordinate.y)
        .min()
        .unwrap_or_else(i32::min_value);

    Vec2 { x: min_x, y: min_y }
}

fn max_boundary(lights: &[Light]) -> Vec2 {
    let max_x = lights
        .iter()
        .map(|light| light.coordinate.x)
        .max()
        .unwrap_or_else(i32::max_value);

    let max_y = lights
        .iter()
        .map(|light| light.coordinate.y)
        .max()
        .unwrap_or_else(i32::max_value);

    Vec2 { x: max_x, y: max_y }
}

fn print_message(lights: &[Light]) {
    let min = min_boundary(&lights);
    let max = max_boundary(&lights);

    let num_x: usize = (max.x - min.x).abs() as usize + 1;
    let num_y: usize = (max.y - min.y).abs() as usize + 1;

    let mut message = vec![vec!['.'; num_x]; num_y];
    for light in lights {
        let x = (light.coordinate.x - min.x) as usize;
        let y = (light.coordinate.y - min.y) as usize;

        message[y][x] = '#';
    }

    for row in message {
        let skip = row.iter().all(|symbol| symbol == &'.');
        if !skip {
            for column in row {
                print!("{}", &column);
            }
            println!()
        }
    }
}

impl Light {
    fn update(&mut self) {
        self.coordinate.x += self.velocity.x;
        self.coordinate.y += self.velocity.y;
    }

    fn step_back(&mut self) {
        self.coordinate.x -= self.velocity.x;
        self.coordinate.y -= self.velocity.y;
    }
}

impl FromStr for Light {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        lazy_static! {
            static ref LIGHT_RE: Regex = Regex::new(
                r"^position=<\s*(?P<position_x>-?\d+),\s*(?P<position_y>-?\d+)>\s*velocity=<\s*(?P<velocity_x>-?\d+),\s*(?P<velocity_y>-?\d+)>"
            )
            .expect("Predefined light regex failed to compile.");
        }

        let capture = LIGHT_RE
            .captures(input)
            .ok_or_else(|| format!("Couldn't parse light from line: {}", input))?;

        let position_x = capture["position_x"].parse()?;
        let position_y = capture["position_y"].parse()?;
        let coordinate = Vec2 {
            x: position_x,
            y: position_y,
        };

        let velocity_x = capture["velocity_x"].parse()?;
        let velocity_y = capture["velocity_y"].parse()?;
        let velocity = Vec2 {
            x: velocity_x,
            y: velocity_y,
        };

        Ok(Light {
            coordinate,
            velocity,
        })
    }
}
