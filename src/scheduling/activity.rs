//! Scheduling activity and time interval abstractions.

/// A discrete time interval `[start, end]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval {
    /// Start time.
    pub start: usize,
    /// End time (inclusive or exclusive depending on convention; start + duration).
    pub end: usize,
}

impl Interval {
    /// Creates a new `Interval`.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns `true` if this interval overlaps with `other`.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Returns the length / duration of the interval.
    pub fn duration(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

/// A schedulable activity/task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activity<ID> {
    /// Activity identifier.
    pub id: ID,
    /// Required execution duration.
    pub duration: usize,
    /// Earliest possible release time.
    pub release_time: usize,
    /// Latest possible completion deadline.
    pub deadline: usize,
}

impl<ID> Activity<ID> {
    /// Creates a new `Activity`.
    pub fn new(id: ID, duration: usize, release_time: usize, deadline: usize) -> Self {
        Self {
            id,
            duration,
            release_time,
            deadline,
        }
    }
}
