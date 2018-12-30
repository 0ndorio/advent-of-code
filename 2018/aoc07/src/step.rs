use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::HashSet,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

#[derive(Ord, PartialOrd, Eq, Clone)]
pub struct StepCell(pub Rc<RefCell<Step>>);

impl StepCell {
    pub fn new(step: Step) -> Self {
        Self {
            0: Rc::new(RefCell::new(step)),
        }
    }
}

impl PartialEq for StepCell {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other)
    }
}

impl Hash for StepCell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl Deref for StepCell {
    type Target = Rc<RefCell<Step>>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct Step {
    pub identifier: char,
    pub depends_on: HashSet<StepCell>,
    pub parent_for: HashSet<StepCell>,
}

impl Step {
    pub fn new(identifier: char) -> Self {
        Self {
            identifier,
            depends_on: HashSet::new(),
            parent_for: HashSet::new(),
        }
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.identifier.eq(&other.identifier)
    }
}

impl Eq for Step {}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.identifier.partial_cmp(&other.identifier)
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.identifier.cmp(&other.identifier)
    }
}

impl Hash for Step {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}
