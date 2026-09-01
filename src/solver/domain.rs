//! Finite Domain representation and domain reduction operations.

use std::collections::HashSet;
use std::hash::Hash;

/// Finite domain of values for a CSP variable.
#[derive(Debug, Clone)]
pub struct Domain<Val> {
    values: HashSet<Val>,
}

impl<Val: Eq + Hash> PartialEq for Domain<Val> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<Val: Eq + Hash> Eq for Domain<Val> {}

impl<Val: Eq + Hash + Clone> Domain<Val> {
    /// Creates a domain from a collection of values.
    pub fn new(values: impl IntoIterator<Item = Val>) -> Self {
        Self {
            values: values.into_iter().collect(),
        }
    }

    /// Returns `true` if the domain contains `val`.
    pub fn contains(&self, val: &Val) -> bool {
        self.values.contains(val)
    }

    /// Returns the number of values remaining in the domain.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the domain is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns an iterator over values in the domain.
    pub fn iter(&self) -> impl Iterator<Item = &Val> {
        self.values.iter()
    }

    /// Prunes `val` from the domain. Returns `true` if domain changed.
    pub fn remove(&mut self, val: &Val) -> bool {
        self.values.remove(val)
    }

    /// Retains only values satisfying `predicate`. Returns `true` if domain changed.
    pub fn retain(&mut self, mut predicate: impl FnMut(&Val) -> bool) -> bool {
        let prev_len = self.values.len();
        self.values.retain(|v| predicate(v));
        self.values.len() != prev_len
    }

    /// Returns the single assigned value if domain has size 1.
    pub fn singleton(&self) -> Option<Val> {
        if self.values.len() == 1 {
            self.values.iter().next().cloned()
        } else {
            None
        }
    }
}
