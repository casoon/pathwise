//! Hill Climbing Local Search.

use crate::core::problem::OptimizationProblem;
use crate::core::solution::{SearchMetrics, Solution};

/// Options for Hill Climbing algorithm execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HillClimbingOptions {
    /// Maximum number of local search iterations.
    pub max_iterations: Option<usize>,
}

/// Optimizes a solution state using Hill Climbing (Steepest Ascent).
///
/// Hill Climbing evaluates neighbors of the current state and moves to the neighboring state
/// with the strictly highest score until no superior neighbor exists (local optimum reached).
///
/// # Complexity
///
/// - **Time Complexity**: $O(k \cdot b)$ where $k$ is number of steps to local optimum and $b$ is neighborhood size.
/// - **Space Complexity**: $O(b)$ to generate and compare immediate neighbors.
///
/// # Requirements
///
/// - `P::Score` must implement `Ord + Copy`.
///
/// # Prefer this when
///
/// - Fast local optimization is required from a good initial state.
/// - The search landscape is unimodal or smooth.
///
/// # Consider instead
///
/// - [`simulated_annealing`](crate::optimization::simulated_annealing()) when local optima traps are common.
///
/// # References
///
/// - Russell, S., & Norvig, P. (2020). *Artificial Intelligence: A Modern Approach* (4th ed.). Pearson. Chapter 4.1.
pub fn hill_climbing<P>(
    problem: &P,
    options: HillClimbingOptions,
) -> Solution<P::State, P::Move, P::Score>
where
    P: OptimizationProblem,
{
    let mut current_state = problem.initial();
    let mut current_score = problem.score(&current_state);
    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;
    let mut iterations = 0;

    loop {
        if options.max_iterations.is_some_and(|max| iterations >= max) {
            break;
        }

        nodes_expanded += 1;
        iterations += 1;

        let mut best_neighbor = None;
        let mut best_neighbor_score = current_score;

        for mv in problem.moves(&current_state) {
            let neighbor = problem.apply(&current_state, &mv);
            nodes_visited += 1;
            let neighbor_score = problem.score(&neighbor);

            if neighbor_score > best_neighbor_score {
                best_neighbor_score = neighbor_score;
                best_neighbor = Some(neighbor);
            }
        }

        match best_neighbor {
            Some(next_state) => {
                current_state = next_state;
                current_score = best_neighbor_score;
            }
            None => break, // Local optimum reached
        }
    }

    Solution::new(
        current_state,
        Vec::new(),
        current_score,
        SearchMetrics {
            nodes_expanded,
            nodes_visited,
        },
    )
}
