use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::collections::btree_set::IntoIter;
use std::collections::btree_set::Iter;

pub struct CustomSet<T> {
    inner_set: BTreeSet<T>
}

impl <T: Ord> IntoIterator for CustomSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner_set.into_iter()
    }
}

impl <T: Ord> FromIterator<T> for CustomSet<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iterator: I) -> Self {
        CustomSet { inner_set: BTreeSet::from_iter(iterator) }
    }
}

impl <T: Ord + Clone> CustomSet<T> {
    pub fn new() -> CustomSet<T> {
        CustomSet { inner_set: BTreeSet::new() }
    }

    pub fn iter(&self) -> Iter<T> {
        self.inner_set.iter()
    }

    pub fn insert(&mut self, element: T) -> bool {
        self.inner_set.insert(element)
    }

    pub fn remove(&mut self, element: &T) -> bool {
        self.inner_set.remove(element)
    }

    pub fn clear(&mut self) {
        self.inner_set.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.inner_set.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner_set.len()
    }

    pub fn contains(&self, element: &T) -> bool {
        self.inner_set.contains(element)
    }

    pub fn is_disjoint(&self, other_set: &CustomSet<T>) -> bool {
        self.inner_set.is_disjoint(&other_set.inner_set)
    }

    pub fn is_subset(&self, other_set: &CustomSet<T>) -> bool {
        self.inner_set.is_subset(&other_set.inner_set)
    }

    pub fn is_superset(&self, other_set: &CustomSet<T>) -> bool {
        self.inner_set.is_superset(&other_set.inner_set)
    }

    pub fn difference(self, other_set: &CustomSet<T>) -> Vec<T> {
        self.inner_set.difference(&other_set.inner_set).cloned().collect()
    }

    pub fn intersection(&self, other_set: &CustomSet<T>) -> Vec<T> {
        self.inner_set.intersection(&other_set.inner_set).cloned().collect()
    }

    pub fn union(&self, other_set: &CustomSet<T>) -> Vec<T> {
        self.inner_set.union(&other_set.inner_set).cloned().collect()
    }
}
