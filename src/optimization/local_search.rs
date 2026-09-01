//! Local Search with Random Restarts.

use crate::core::problem::OptimizationProblem;
use crate::core::solution::{SearchMetrics, Solution};
use crate::optimization::coordination::Coordination;
use crate::optimization::hill_climbing::{HillClimbingOptions, hill_climbing};

/// Options for local search with restarts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSearchOptions {
    /// Number of random restarts / optimization passes to run.
    pub restarts: usize,
    /// Options for each hill climbing run.
    pub hill_climbing_options: HillClimbingOptions,
}

impl Default for LocalSearchOptions {
    fn default() -> Self {
        Self {
            restarts: 5,
            hill_climbing_options: HillClimbingOptions::default(),
        }
    }
}

/// Runs local search with restarts to find high-scoring state configurations.
///
/// Executes multiple runs of hill climbing from different state initializations,
/// retaining the globally best candidate solution found across all runs.
///
/// # Complexity
///
/// - **Time Complexity**: $O(R \cdot k \cdot b)$ where $R$ is restart count, $k$ is steps, $b$ is neighborhood size.
/// - **Space Complexity**: $O(b)$ bounded memory.
///
/// # Requirements
///
/// - `P::Score` must implement `Ord + Copy`.
///
/// # Prefer this when
///
/// - The search landscape has multiple local optima and standard hill climbing gets stuck easily.
///
/// # References
///
/// - Russell, S., & Norvig, P. (2020). *Artificial Intelligence: A Modern Approach* (4th ed.). Pearson. Chapter 4.1.2.
///
/// The first restart always runs (this function returns a `Solution`, not an `Option`). If
/// `coordination.cancellation` is set (see [`crate::core::cancellation::CancellationToken`]), it
/// is checked once per subsequent restart, and the search returns early with the best solution
/// found so far. If `coordination.incumbent` is set (see
/// [`crate::core::incumbent::SharedIncumbent`]), every improvement to `best_solution` (including
/// the first restart's result) is offered to it.
pub fn local_search<P>(
    problem: &P,
    options: LocalSearchOptions,
    coordination: &Coordination<P::State, P::Score>,
) -> Solution<P::State, P::Move, P::Score>
where
    P: OptimizationProblem,
{
    let mut total_expanded = 0;
    let mut total_visited = 0;

    let mut best_solution = hill_climbing(problem, options.hill_climbing_options);
    total_expanded += best_solution.metrics.nodes_expanded;
    total_visited += best_solution.metrics.nodes_visited;
    if let Some(incumbent) = &coordination.incumbent {
        incumbent.offer(&best_solution.state, best_solution.cost);
    }

    for _ in 1..options.restarts {
        if coordination
            .cancellation
            .as_ref()
            .is_some_and(|token| token.is_cancelled())
        {
            break;
        }

        let sol = hill_climbing(problem, options.hill_climbing_options);
        total_expanded += sol.metrics.nodes_expanded;
        total_visited += sol.metrics.nodes_visited;

        if sol.cost > best_solution.cost {
            best_solution = sol;
            if let Some(incumbent) = &coordination.incumbent {
                incumbent.offer(&best_solution.state, best_solution.cost);
            }
        }
    }

    best_solution.metrics = SearchMetrics {
        nodes_expanded: total_expanded,
        nodes_visited: total_visited,
    };

    best_solution
}
