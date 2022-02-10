use std::iter::FromIterator;
use std::slice::Iter;

#[derive(Clone, Debug)]
pub struct CustomSet<T: PartialEq + Clone> {
    set: Vec<T>,
}

// This trait implementation is very important.
// Not only is it required for the exercise,
// but we're going to make great use of it when implementing the set mechanics.
impl<T: PartialEq + Clone> FromIterator<T> for CustomSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        let mut set = CustomSet { set: vec![] };
        for item in it {
            set.add(item);
        }
        set
    }
}

/// Simple implementation of Set semantics.
impl<T: PartialEq + Clone> CustomSet<T> {
    /* The base data type is a Vec, which gives us some methods for free. */
    pub fn clear(&mut self) {
        self.set.clear()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.set.contains(item)
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn iter(&self) -> Iter<T> {
        self.set.iter()
    }

    /* The missing functions can be defined based on the above, other Vec basics and eachother. */
    pub fn remove(&mut self, item: &T) -> bool {
        self.iter()
            .position(|e| e == item)
            .map(|n| self.set.swap_remove(n))
            .is_some()
    }

    pub fn add(&mut self, item: T) -> bool {
        (!self.contains(&item))
            .then(|| self.set.push(item))
            .is_some()
    }

    pub fn difference(&self, other: &Self) -> Self {
        self.iter()
            .filter(|e| !other.contains(e))
            .cloned()
            .collect()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        self.iter().filter(|e| other.contains(e)).cloned().collect()
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.iter().all(|e| other.contains(e))
    }

    pub fn union(&self, other: &Self) -> Self {
        self.iter().chain(other.iter()).cloned().collect()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(&self)
    }

    pub fn new(contents: &[T]) -> Self {
        contents.iter().cloned().collect()
    }
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &CustomSet<T>) -> bool {
        self.is_subset(other) && self.len() == other.len()
    }
}
