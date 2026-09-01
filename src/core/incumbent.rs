//! Thread-safe shared incumbent for portfolio search coordination.
//!
//! Lets independently-running search/optimization workers both contribute improving solutions
//! and consult the best one found so far by *any* worker — e.g. a fast, incomplete algorithm
//! (Local Search, LNS) can hand Branch & Bound a strong starting bound instead of it beginning
//! from scratch.
//!
//! Reference:
//! - Gomes, C. P., & Selman, B. (2001). *Algorithm portfolios*. Artificial Intelligence, 126(1-2), 43-62.

use std::sync::{Arc, Mutex};

/// The best `(state, score)` pair found so far across all portfolio workers sharing this handle,
/// if any. Cloning shares the same underlying state (see [`crate::core::cancellation::CancellationToken`]
/// for the identical pattern).
#[derive(Debug, Clone)]
pub struct SharedIncumbent<S: Clone, Score: Ord + Copy> {
    best: Arc<Mutex<Option<(S, Score)>>>,
}

impl<S: Clone, Score: Ord + Copy> Default for SharedIncumbent<S, Score> {
    fn default() -> Self {
        Self {
            best: Arc::new(Mutex::new(None)),
        }
    }
}

impl<S: Clone, Score: Ord + Copy> SharedIncumbent<S, Score> {
    /// Creates an empty shared incumbent (no solution found yet).
    ///
    /// Time & Space: O(1).
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current best score, if any.
    ///
    /// # Complexity
    /// Time: O(1) (one mutex lock, no clone of the state).
    pub fn best_score(&self) -> Option<Score> {
        self.best
            .lock()
            .expect("shared incumbent mutex poisoned")
            .as_ref()
            .map(|(_, score)| *score)
    }

    /// Returns a clone of the current best `(state, score)` pair, if any.
    ///
    /// # Complexity
    /// Time & Space: O(N) where N is the state size, to clone it out from behind the lock.
    pub fn best(&self) -> Option<(S, Score)> {
        self.best
            .lock()
            .expect("shared incumbent mutex poisoned")
            .clone()
    }

    /// Replaces the incumbent with `(state, score)` if `score` improves on the current best (or
    /// none exists yet). Returns `true` if it did.
    ///
    /// # Complexity
    /// Time & Space: O(N) where N is the state size (cloned into shared storage on improvement;
    /// O(1) otherwise).
    pub fn offer(&self, state: &S, score: Score) -> bool {
        let mut guard = self.best.lock().expect("shared incumbent mutex poisoned");
        let improves = guard.as_ref().is_none_or(|(_, best)| score > *best);
        if improves {
            *guard = Some((state.clone(), score));
        }
        improves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_accepts_first_solution() {
        let incumbent: SharedIncumbent<i32, i32> = SharedIncumbent::new();
        assert!(incumbent.offer(&1, 5));
        assert_eq!(incumbent.best_score(), Some(5));
    }

    #[test]
    fn test_offer_rejects_non_improving_solution() {
        let incumbent: SharedIncumbent<i32, i32> = SharedIncumbent::new();
        assert!(incumbent.offer(&1, 5));
        assert!(!incumbent.offer(&1, 3));
        assert_eq!(
            incumbent.best_score(),
            Some(5),
            "a worse offer must not overwrite the existing incumbent"
        );
    }

    #[test]
    fn test_offer_accepts_strictly_improving_solution() {
        let incumbent: SharedIncumbent<i32, i32> = SharedIncumbent::new();
        incumbent.offer(&1, 5);
        assert!(incumbent.offer(&1, 7));
        assert_eq!(incumbent.best_score(), Some(7));
    }

    #[test]
    fn test_clone_shares_underlying_state() {
        let incumbent: SharedIncumbent<i32, i32> = SharedIncumbent::new();
        let handle = incumbent.clone();
        handle.offer(&1, 5);
        assert_eq!(
            incumbent.best_score(),
            Some(5),
            "clones must observe each other's updates"
        );
    }
}
