//! Iterative Deepening Depth-First Search (IDDFS).

use std::hash::Hash;

use crate::core::problem::SearchProblem;
use crate::core::solution::Solution;

use crate::search::dfs::{DfsOptions, dfs};

/// Finds the shortest path to a goal using Iterative Deepening Depth-First Search (IDDFS).
///
/// IDDFS repeatedly runs depth-limited DFS with incrementally increasing depth limits $0, 1, 2, \dots, \text{max\_depth}$.
/// It combines the optimal step guarantees of BFS with the linear space complexity of DFS.
///
/// # Complexity
///
/// - **Time Complexity**: $O(b^d)$ where $b$ is branching factor and $d$ is goal depth.
/// - **Space Complexity**: $O(b \cdot d)$ linear space complexity.
///
/// # Requirements
///
/// - `P::State` must implement `Eq + Hash + Clone`.
///
/// # Prefer this when
///
/// - Search space is large or infinite, memory is limited, and shortest path is needed.
///
/// # Consider instead
///
/// - [`astar`](crate::search::astar()) when an informative heuristic is available.
/// - [`bfs`](crate::search::bfs()) when memory overhead is not a concern.
///
/// # References
///
/// - Korf, R. E. (1985). Depth-first iterative-deepening: An optimal admissible tree search. *Artificial Intelligence*, 27(1), 97-109.
pub fn iddfs<P>(problem: &P, max_depth_limit: usize) -> Option<Solution<P::State, P::Move, P::Cost>>
where
    P: SearchProblem,
    P::State: Eq + Hash,
{
    let mut total_expanded = 0;
    let mut total_visited = 0;

    for depth in 0..=max_depth_limit {
        let options = DfsOptions {
            max_depth: Some(depth),
        };
        if let Some(mut sol) = dfs(problem, options) {
            sol.metrics.nodes_expanded += total_expanded;
            sol.metrics.nodes_visited += total_visited;
            return Some(sol);
        } else {
            // Rough metrics estimation across depth levels
            total_expanded += depth;
            total_visited += depth * 2;
        }
    }

    None
}
