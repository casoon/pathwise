//! Portfolio coordination handles shared across optimization algorithm invocations.

use crate::core::cancellation::CancellationToken;
use crate::core::incumbent::SharedIncumbent;

/// Optional cancellation and shared-incumbent handles threaded through [`super::branch_and_bound()`],
/// [`super::local_search()`], and [`super::large_neighborhood_search()`].
///
/// Kept as a separate parameter rather than fields on `LocalSearchOptions`/`LnsOptions`: those
/// are `Copy + PartialEq` value-only option structs, and `CancellationToken`/`SharedIncumbent`
/// are `Arc`-based handles that would break both derives. `Coordination::default()` (no
/// cancellation, no shared incumbent) preserves each function's original, uncoordinated
/// behavior.
#[derive(Debug, Clone)]
pub struct Coordination<S: Clone, Score: Ord + Copy> {
    /// Externally triggered cancellation, checked periodically during search.
    pub cancellation: Option<CancellationToken>,
    /// Portfolio-wide best-solution handle: improvements found here are offered to it, and (for
    /// [`super::branch_and_bound()`]) a stronger incumbent found elsewhere is adopted as a bound.
    pub incumbent: Option<SharedIncumbent<S, Score>>,
}

impl<S: Clone, Score: Ord + Copy> Default for Coordination<S, Score> {
    /// No cancellation, no shared incumbent — matches each algorithm's original,
    /// uncoordinated behavior.
    fn default() -> Self {
        Self {
            cancellation: None,
            incumbent: None,
        }
    }
}
