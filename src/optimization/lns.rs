//! Large Neighborhood Search (LNS).

use crate::core::problem::OptimizationProblem;
use crate::core::solution::{SearchMetrics, Solution};
use crate::optimization::coordination::Coordination;

/// Trait defining destroy and repair operators for Large Neighborhood Search.
pub trait LnsProblem: OptimizationProblem {
    /// Partial solution representation during destroy/repair.
    type PartialState;

    /// Destroy operator: removes or unassigns parts of `state`.
    fn destroy(&self, state: &Self::State, intensity: f64) -> Self::PartialState;

    /// Repair operator: reconstructs a feasible `State` from `partial`.
    fn repair(&self, partial: &Self::PartialState) -> Self::State;
}

/// Options configuring Large Neighborhood Search execution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LnsOptions {
    /// Maximum search iterations.
    pub max_iterations: usize,
    /// Neighborhood destruction intensity $\in (0, 1)$.
    pub destroy_intensity: f64,
}

impl Default for LnsOptions {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            destroy_intensity: 0.3,
        }
    }
}

/// Optimizes state configurations using Large Neighborhood Search (LNS).
///
/// LNS explores large structural neighborhoods by destroying parts of a solution and repairing them.
///
/// # Complexity
///
/// - **Time Complexity**: $O(k \cdot (C_{\text{destroy}} + C_{\text{repair}}))$ where $k$ is iteration count.
/// - **Space Complexity**: $O(\text{state size})$.
///
/// # References
///
/// - Shaw, P. (1998). Using constraint programming and local search methods to solve vehicle routing problems. *Principles and Practice of Constraint Programming*, 417-431.
///
/// If `coordination.cancellation` is set (see [`crate::core::cancellation::CancellationToken`]),
/// it is checked once per iteration, and the search returns early with the best solution found
/// so far. If `coordination.incumbent` is set (see [`crate::core::incumbent::SharedIncumbent`]),
/// every improvement to `best_score` (including the initial state) is offered to it.
pub fn large_neighborhood_search<P>(
    problem: &P,
    options: LnsOptions,
    coordination: &Coordination<P::State, P::Score>,
) -> Solution<P::State, P::Move, P::Score>
where
    P: LnsProblem,
{
    let mut current_state = problem.initial();
    let mut current_score = problem.score(&current_state);
    let mut best_state = current_state.clone();
    let mut best_score = current_score;
    if let Some(incumbent) = &coordination.incumbent {
        incumbent.offer(&best_state, best_score);
    }

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    for _ in 0..options.max_iterations {
        if coordination
            .cancellation
            .as_ref()
            .is_some_and(|token| token.is_cancelled())
        {
            break;
        }

        nodes_expanded += 1;

        let partial = problem.destroy(&current_state, options.destroy_intensity);
        let neighbor = problem.repair(&partial);
        nodes_visited += 1;
        let neighbor_score = problem.score(&neighbor);

        if neighbor_score > current_score {
            current_state = neighbor;
            current_score = neighbor_score;

            if current_score > best_score {
                best_state = current_state.clone();
                best_score = current_score;
                if let Some(incumbent) = &coordination.incumbent {
                    incumbent.offer(&best_state, best_score);
                }
            }
        }
    }

    Solution::new(
        best_state,
        Vec::new(),
        best_score,
        SearchMetrics {
            nodes_expanded,
            nodes_visited,
        },
    )
}
