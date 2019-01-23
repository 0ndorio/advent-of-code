use std::{
    cell::RefCell,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
    thread,
    time::Duration,
};

use crate::{
    location::Location,
    map::Map,
    player::{Player, Race},
    tile::Tile,
};

#[derive(Debug, Clone)]
pub struct Game {
    map: Map,
    player: Vec<Rc<RefCell<Player>>>,
}

#[derive(Debug)]
pub enum GameResult {
    NotYetDone,
    Finished(Race),
}

impl Game {
    pub fn new(map: Map) -> Self {
        let player = map.find_player();
        Self { map, player }
    }

    pub fn count_elves(&self) -> usize {
        self.player
            .iter()
            .filter(|player| player.borrow().race == Race::Elf)
            .filter(|player| player.borrow().alive())
            .count()
    }

    pub fn set_elf_attack_power(&self, ap: u32) {
        self.player
            .iter()
            .filter(|player| player.borrow().race == Race::Elf)
            .for_each(|elf| elf.borrow_mut().attack_power = ap);
    }

    pub fn run(&mut self) -> (u32, Race, u32) {
        self.draw_map();

        let mut turn_counter = 0;

        let winner = loop {
            let result = self.next_turn();
            self.draw_map();

            if let GameResult::Finished(race) = result {
                break race;
            }

            turn_counter += 1;
        };

        let hit_points_left = self
            .map
            .find_player()
            .into_iter()
            .map(|player| player.borrow().hit_points)
            .sum();

        (turn_counter, winner, hit_points_left)
    }

    pub fn next_turn(&mut self) -> GameResult {
        for player in self.map.find_player() {
            if !player.borrow().alive() {
                continue;
            }

            {
                let current_race = player.borrow().race;
                if self.has_race_won(current_race) {
                    return GameResult::Finished(current_race);
                }
            }

            let nearby_target = {
                let player = player.borrow();
                self.get_nearby_target(&player)
            };

            if nearby_target.is_none() {
                self.move_player(&player)
            }

            let nearby_target = {
                let player = player.borrow();
                self.get_nearby_target(&player)
            };

            if let Some(target) = nearby_target {
                let player = player.borrow();
                let mut target = target.borrow_mut();

                let is_dead = player.attack(&mut target);
                if is_dead {
                    self.map.tiles.insert(target.location.clone(), Tile::Floor);
                }
            }
        }

        GameResult::NotYetDone
    }

    fn get_nearby_target(&self, player: &Player) -> Option<Rc<RefCell<Player>>> {
        player
            .location
            .adjacent()
            .into_iter()
            .flat_map(|location| self.map.tiles.get(&location))
            .flat_map(Tile::as_player)
            .filter(|other_player| other_player.borrow().race != player.race)
            .min_by_key(|target| target.borrow().hit_points)
    }

    fn move_player(&mut self, player_ref: &Rc<RefCell<Player>>) {
        let mut player = player_ref.borrow_mut();
        let distance_map = self.generate_distance_map(&player.location);

        let mut potential_targets = HashMap::new();
        for target in self.map.find_player() {
            if Rc::ptr_eq(&target, &player_ref) {
                continue;
            }

            let target = target.borrow();
            if player.race == target.race {
                continue;
            }

            let target_locations = self.map.free_adjacent(&target.location);
            for target_location in target_locations {
                if let Some(&distance) = distance_map.get(&target_location) {
                    let entry = potential_targets.entry(distance).or_insert_with(|| vec![]);
                    entry.push(target_location)
                }
            }
        }

        if let Some(min_distance) = potential_targets.keys().min().cloned() {
            if let Some(potential_targets) = potential_targets.remove(&min_distance) {
                let mut potential_targets = potential_targets;
                potential_targets.sort();

                if let Some(target) = potential_targets.first() {
                    let path = self
                        .calc_path(&player.location, target, &distance_map)
                        .expect("There must no empty paths exist.");

                    let new_location = path[path.len() - 1].clone();

                    self.map.tiles.insert(player.location.clone(), Tile::Floor);
                    self.map
                        .tiles
                        .insert(new_location.clone(), Tile::Player(Rc::clone(player_ref)));

                    (*player).location = new_location.clone();
                }
            }
        }
    }

    pub fn generate_distance_map(&self, location: &Location) -> HashMap<Location, u32> {
        let mut distance_map = HashMap::new();

        let mut waiting = HashSet::new();

        let mut outstanding_locations = VecDeque::new();
        outstanding_locations.push_back((location.clone(), 0u32));

        while let Some((location, distance)) = outstanding_locations.pop_front() {
            self.map
                .free_adjacent(&location)
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
        distance_map: &HashMap<Location, u32>,
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

            let mut next_locations = next_locations
                .into_iter()
                .filter(|adjacent| self.map.is_free(adjacent))
                .filter(|adjacent| !visited.contains(adjacent))
                .filter(|adjacent| {
                    let distance = distance_map.get(adjacent);
                    distance < current_distance
                })
                .collect::<Vec<_>>();

            next_locations.sort();
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
        self.map
            .find_player()
            .into_iter()
            .all(|other_player| other_player.borrow().race == race)
    }

    fn draw_map(&self) {
        // clear screen and reset to top left; draw an "animation"
        print!("{}[2J{}[H{}", 27 as char, 27 as char, self.map);
        thread::sleep(Duration::from_millis(10));
    }
}
