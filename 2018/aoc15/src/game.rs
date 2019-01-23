use std::{
    cell::RefCell,
    collections::{BTreeMap, BinaryHeap, HashSet, VecDeque},
    error::Error,
    fmt::{self, Display, Formatter},
    rc::Rc,
    str::FromStr,
    thread,
    time::Duration,
};

use crate::{
    location::Location,
    map::Map,
    tile::Tile,
    unit::{Race, Unit},
    Result,
};

pub type UnitHandle = Rc<RefCell<Unit>>;

#[derive(Debug, Clone)]
pub struct Game {
    map: Map,
    units: BTreeMap<Location, UnitHandle>,
}

#[derive(Debug)]
pub enum GameResult {
    NotYetDone,
    Finished(Race),
}

impl Game {
    pub fn count_elves(&self) -> usize {
        self.units
            .values()
            .filter(|unit| unit.borrow().race == Race::Elf)
            .filter(|unit| unit.borrow().alive())
            .count()
    }

    pub fn set_elf_attack_power(&self, ap: u32) {
        self.units
            .values()
            .filter(|unit| unit.borrow().race == Race::Elf)
            .for_each(|elf| elf.borrow_mut().attack_power = ap);
    }

    pub fn run(&mut self) -> (u32, Race, u32) {
        self.draw();

        let mut turn_counter = 0;

        let winner = loop {
            let result = self.next_turn();
            self.draw();

            if let GameResult::Finished(race) = result {
                break race;
            }

            turn_counter += 1;
        };

        let hit_points_left = self
            .units
            .values()
            .map(|unit| unit.borrow().hit_points)
            .sum();

        (turn_counter, winner, hit_points_left)
    }

    pub fn next_turn(&mut self) -> GameResult {
        let units: Vec<UnitHandle> = self.units.values().map(UnitHandle::clone).collect();
        for unit in units {
            if !unit.borrow().alive() {
                continue;
            }

            {
                let current_race = unit.borrow().race;
                if self.has_race_won(current_race) {
                    return GameResult::Finished(current_race);
                }
            }

            let nearby_target = {
                let unit = unit.borrow();
                self.get_nearby_target(&unit)
            };

            if nearby_target.is_none() {
                self.move_unit(&unit)
            }

            let nearby_target = {
                let unit = unit.borrow();
                self.get_nearby_target(&unit)
            };

            if let Some(target) = nearby_target {
                let unit = unit.borrow();
                let mut target = target.borrow_mut();

                let is_dead = unit.attack(&mut target);
                if is_dead {
                    self.units.remove(&target.location);
                }
            }
        }

        GameResult::NotYetDone
    }

    fn get_nearby_target(&self, unit: &Unit) -> Option<UnitHandle> {
        unit.location
            .adjacent()
            .into_iter()
            .flat_map(|location| self.units.get(&location))
            .filter(|other_unit| other_unit.borrow().race != unit.race)
            .min_by_key(|target| target.borrow().hit_points)
            .map(UnitHandle::clone)
    }

    fn move_unit(&mut self, unit_ref: &UnitHandle) {
        let mut unit = unit_ref.borrow_mut();
        let distance_map = self.generate_distance_map(&unit.location);

        let mut potential_targets = BTreeMap::new();
        for target in self.units.values() {
            if UnitHandle::ptr_eq(&target, &unit_ref) {
                continue;
            }

            let target = target.borrow();
            if unit.race == target.race {
                continue;
            }

            let target_locations = self.free_adjacent(&target.location);
            for target_location in target_locations {
                if let Some(&distance) = distance_map.get(&target_location) {
                    let entry = potential_targets.entry(distance).or_insert_with(|| vec![]);
                    entry.push(target_location)
                }
            }
        }

        if let Some(min_distance) = potential_targets.keys().min().cloned() {
            if let Some(potential_targets) = potential_targets.remove(&min_distance) {
                if let Some(target) = potential_targets.first() {
                    let path = self
                        .calc_path(&unit.location, target, &distance_map)
                        .expect("There must no empty paths exist.");

                    let new_location = path[path.len() - 1].clone();

                    self.units.remove(&unit.location);
                    self.units
                        .insert(new_location.clone(), UnitHandle::clone(unit_ref));

                    (*unit).location = new_location.clone();
                }
            }
        }
    }

    pub fn generate_distance_map(&self, location: &Location) -> BTreeMap<Location, u32> {
        let mut distance_map = BTreeMap::new();
        let mut waiting = HashSet::new();

        let mut outstanding_locations = VecDeque::new();
        outstanding_locations.push_back((location.clone(), 0u32));

        while let Some((location, distance)) = outstanding_locations.pop_front() {
            self.free_adjacent(&location)
                .into_iter()
                .filter(|adjacent| !distance_map.contains_key(adjacent))
                .for_each(|adjacent| {
                    if !waiting.contains(&adjacent) {
                        waiting.insert(adjacent.clone());
                        outstanding_locations.push_back((adjacent, distance + 1));
                    }
                });

            waiting.remove(&location);
            distance_map.insert(location, distance + 1);
        }

        distance_map
    }

    /// Calculate a path between start and destination based on a pre calculated distance map
    /// which must be based on the start location.
    fn calc_path(
        &self,
        start: &Location,
        destination: &Location,
        distance_map: &BTreeMap<Location, u32>,
    ) -> Option<Vec<Location>> {
        if !distance_map.contains_key(&start) {
            return None;
        }

        if !distance_map.contains_key(&destination) {
            return None;
        }

        let mut path = vec![];
        let mut last_element_distance = Some(&u32::max_value());

        let mut visited = HashSet::new();
        let mut outstanding = BinaryHeap::new();
        outstanding.push(destination.clone());

        while let Some(location) = outstanding.pop() {
            let current_distance = distance_map.get(&location);

            let next_locations = location.adjacent();
            if next_locations.contains(start) {
                path.push((location, current_distance));
                break;
            }

            while current_distance >= last_element_distance {
                path.pop();

                if let Some((_, distance)) = path.last() {
                    last_element_distance = *distance;
                } else {
                    last_element_distance = Some(&u32::max_value());
                }
            }

            let next_locations = next_locations
                .into_iter()
                .filter(|adjacent| self.map.is_free(adjacent))
                .filter(|adjacent| !self.units.contains_key(adjacent))
                .filter(|adjacent| !visited.contains(adjacent))
                .filter(|adjacent| {
                    let distance = distance_map.get(adjacent);
                    distance < current_distance
                })
                .collect::<Vec<_>>();

            if let Some(next_location) = next_locations.first() {
                outstanding.push(next_location.clone());
                path.push((location.clone(), current_distance));
            }

            visited.insert(location);
        }

        let path = path.into_iter().map(|(location, _)| location).collect();
        Some(path)
    }

    fn has_race_won(&self, race: Race) -> bool {
        self.units
            .values()
            .all(|other_unit| other_unit.borrow().race == race)
    }

    fn draw(&self) {
        // clear screen and reset to top left; draw an "animation"
        print!("{}[2J{}[H{}", 27 as char, 27 as char, self);
        thread::sleep(Duration::from_millis(50));
    }

    pub fn free_adjacent(&self, location: &Location) -> Vec<Location> {
        self.map
            .free_adjacent(location)
            .into_iter()
            .filter(|adjacent| !self.units.contains_key(adjacent))
            .collect()
    }
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let raw_tiles = input.lines().enumerate().flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, symbol)| (Location::new(x, y), symbol))
        });

        let mut tiles = BTreeMap::new();
        let mut units = BTreeMap::new();

        for (location, symbol) in raw_tiles {
            match symbol {
                'G' | 'E' => {
                    let mut unit = Unit::from_char(symbol)?;
                    unit.location = location.clone();
                    units.insert(location.clone(), Rc::new(RefCell::new(unit)));
                    tiles.insert(location, Tile::Floor);
                }
                '.' | '#' => {
                    let tile = Tile::from_char(symbol)?;
                    tiles.insert(location, tile);
                }
                _ => Err(format!("Unknown symbol discovered: {}", symbol))?,
            }
        }

        let map = Map::new(tiles);

        Ok(Game { map, units })
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let width = self.map.size.0;

        for (location, tile) in &self.map.tiles {
            let symbol = if let Some(unit) = self.units.get(location) {
                unit.borrow().to_string()
            } else {
                tile.to_string()
            };

            if (location.x + 1) == width {
                writeln!(f, "{}", symbol)?;
            } else {
                write!(f, "{}", symbol)?;
            }
        }

        Ok(())
    }
}
