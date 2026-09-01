//! Tabu Search Optimization.

use std::collections::VecDeque;
use std::hash::Hash;

use crate::core::problem::OptimizationProblem;
use crate::core::solution::{SearchMetrics, Solution};

/// Options configuring Tabu Search execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabuSearchOptions {
    /// Maximum number of iterations to perform.
    pub max_iterations: usize,
    /// Size limit of the short-term Tabu list ($T$).
    pub tabu_tenure: usize,
}

impl Default for TabuSearchOptions {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            tabu_tenure: 10,
        }
    }
}

/// Optimizes state solutions using Tabu Search.
///
/// Tabu Search avoids cycling and escaping local optima by maintaining a memory FIFO queue (Tabu list)
/// of recently visited states, overriding tabu restrictions only if an aspiration criterion is satisfied.
///
/// # Complexity
///
/// - **Time Complexity**: $O(k \cdot b)$ where $k$ is iterations and $b$ is move neighborhood size.
/// - **Space Complexity**: $O(T + b)$ where $T$ is tabu tenure.
///
/// # Requirements
///
/// - `P::State` must implement `Eq + Hash + Clone`.
/// - `P::Score` must implement `Ord + Copy`.
///
/// # Prefer this when
///
/// - Local search landscape has local optima traps or plateau loops.
///
/// # References
///
/// - Glover, F. (1989). Tabu search—Part I. *ORSA Journal on Computing*, 1(3), 190-206.
pub fn tabu_search<P>(
    problem: &P,
    options: TabuSearchOptions,
) -> Solution<P::State, P::Move, P::Score>
where
    P: OptimizationProblem,
    P::State: Eq + Hash,
{
    let mut current_state = problem.initial();
    let mut current_score = problem.score(&current_state);
    let mut best_state = current_state.clone();
    let mut best_score = current_score;

    let mut tabu_list: VecDeque<P::State> = VecDeque::with_capacity(options.tabu_tenure);
    tabu_list.push_back(current_state.clone());

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    for _ in 0..options.max_iterations {
        nodes_expanded += 1;

        let mut best_candidate = None;
        let mut best_candidate_score = None;

        for mv in problem.moves(&current_state) {
            let neighbor = problem.apply(&current_state, &mv);
            nodes_visited += 1;
            let neighbor_score = problem.score(&neighbor);

            let is_tabu = tabu_list.contains(&neighbor);
            let satisfies_aspiration = neighbor_score > best_score;

            if !is_tabu || satisfies_aspiration {
                let is_better_candidate = match best_candidate_score {
                    Some(s) => neighbor_score > s,
                    None => true,
                };

                if is_better_candidate {
                    best_candidate = Some(neighbor);
                    best_candidate_score = Some(neighbor_score);
                }
            }
        }

        match (best_candidate, best_candidate_score) {
            (Some(next_state), Some(next_score)) => {
                current_state = next_state.clone();
                current_score = next_score;

                if current_score > best_score {
                    best_state = current_state.clone();
                    best_score = current_score;
                }

                if tabu_list.len() >= options.tabu_tenure {
                    tabu_list.pop_front();
                }
                tabu_list.push_back(next_state);
            }
            _ => break, // No valid non-tabu moves available
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
