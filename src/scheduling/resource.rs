//! Resource capacity and disjunctive scheduling constraints.

use crate::scheduling::activity::Interval;

/// A unary / disjunctive resource that can execute at most one activity at a time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryResource<ID> {
    /// Resource identifier.
    pub id: ID,
}

impl<ID> UnaryResource<ID> {
    /// Creates a new `UnaryResource`.
    pub fn new(id: ID) -> Self {
        Self { id }
    }

    /// Checks if a set of scheduled activity intervals contains any overlapping conflicts.
    pub fn validate_no_overlap(&self, intervals: &[Interval]) -> bool {
        for i in 0..intervals.len() {
            for j in (i + 1)..intervals.len() {
                if intervals[i].overlaps(&intervals[j]) {
                    return false;
                }
            }
        }
        true
    }
}

/// A cumulative resource with a maximum capacity limit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CumulativeResource<ID> {
    /// Resource identifier.
    pub id: ID,
    /// Maximum simultaneous capacity limit.
    pub max_capacity: usize,
}

impl<ID> CumulativeResource<ID> {
    /// Creates a new `CumulativeResource`.
    pub fn new(id: ID, max_capacity: usize) -> Self {
        Self { id, max_capacity }
    }
}
