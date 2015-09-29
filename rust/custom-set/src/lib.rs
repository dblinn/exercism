use std::hash::Hash;
use std::collections::HashSet;

pub struct CustomSet<T> {
    innerSet: HashSet<T>
}

impl <T: Hash + Eq> CustomSet<T> {
    pub fn new() -> CustomSet<T> {
        CustomSet { innerSet: HashSet::new() }
    }

    pub fn insert(&mut self, element: T) {
        self.innerSet.insert(element);
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn contains(&self, element: &T) -> bool {
        false
    }

    pub fn is_disjoint(&self) -> bool {
        false
    }

    pub fn is_subset(&self) -> bool {
        false
    }

    pub fn is_superset(&self) -> bool {
        false
    }
}
