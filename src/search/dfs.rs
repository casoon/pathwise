//! Depth-First Search (DFS).

use std::collections::HashSet;
use std::hash::Hash;

use crate::core::problem::SearchProblem;
use crate::core::solution::{SearchMetrics, Solution};

/// Options for Depth-First Search execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DfsOptions {
    /// Optional maximum depth limit for exploration.
    pub max_depth: Option<usize>,
}

/// Finds a goal state using Depth-First Search (DFS) with optional depth limit.
///
/// DFS explores as deep as possible along each branch before backtracking.
///
/// # Complexity
///
/// - **Time Complexity**: $O(b^m)$ where $b$ is branching factor and $m$ is maximum depth.
/// - **Space Complexity**: $O(b \cdot m)$ for the call stack and path storage (linear in depth).
///
/// # Requirements
///
/// - `P::State` must implement `Eq + Hash + Clone` to prevent cycle trapping.
///
/// # Prefer this when
///
/// - Memory is limited and search trees are deep or infinite (with depth limits).
/// - Many goal states exist and finding *any* solution quickly is sufficient.
///
/// # Consider instead
///
/// - [`bfs`](crate::search::bfs()) when the shortest path is required.
/// - [`iddfs`](crate::search::iddfs()) for optimal depth bounds with linear memory consumption.
///
/// # References
///
/// - Russell, S., & Norvig, P. (2020). *Artificial Intelligence: A Modern Approach* (4th ed.). Pearson. Chapter 3.4.
pub fn dfs<P>(problem: &P, options: DfsOptions) -> Option<Solution<P::State, P::Move, P::Cost>>
where
    P: SearchProblem,
    P::State: Eq + Hash,
{
    let initial_state = problem.initial();
    if problem.is_goal(&initial_state) {
        return Some(Solution::new(
            initial_state,
            Vec::new(),
            P::Cost::default(),
            SearchMetrics {
                nodes_expanded: 0,
                nodes_visited: 1,
            },
        ));
    }

    let mut stack = vec![(initial_state.clone(), Vec::new(), P::Cost::default(), 0)];
    let mut visited = HashSet::new();
    visited.insert(initial_state);

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    while let Some((curr_state, moves, cost, depth)) = stack.pop() {
        nodes_expanded += 1;

        if options.max_depth.is_some_and(|max| depth >= max) {
            continue;
        }

        for mv in problem.moves(&curr_state) {
            let next_state = problem.apply(&curr_state, &mv);
            if visited.contains(&next_state) {
                continue;
            }

            nodes_visited += 1;
            let step_cost = problem.step_cost(&curr_state, &mv);
            let next_cost = cost + step_cost;

            let mut next_moves = moves.clone();
            next_moves.push(mv);

            if problem.is_goal(&next_state) {
                return Some(Solution::new(
                    next_state,
                    next_moves,
                    next_cost,
                    SearchMetrics {
                        nodes_expanded,
                        nodes_visited,
                    },
                ));
            }

            visited.insert(next_state.clone());
            stack.push((next_state, next_moves, next_cost, depth + 1));
        }
    }

    None
}
