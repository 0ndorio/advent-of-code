// Here the order of the struct member is important as it
// decides how partial is derived and we want rows to be more
// important than columns.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct Location {
    pub y: usize,
    pub x: usize,
}

impl Location {
    pub fn new(x: usize, y: usize) -> Self {
        Location { y, x }
    }

    /// Returns adjacent locations in reading-order (top-to-bottom; left-to-right).
    pub fn adjacent(&self) -> Vec<Location> {
        vec![
            Location::new(self.x, self.y - 1),
            Location::new(self.x - 1, self.y),
            Location::new(self.x + 1, self.y),
            Location::new(self.x, self.y + 1),
        ]
    }
}
