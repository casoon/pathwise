//! Breadth-First Search (BFS).

use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use crate::core::problem::SearchProblem;
use crate::core::solution::{SearchMetrics, Solution};

/// Finds the shortest path (in number of moves) to a goal state using Breadth-First Search (BFS).
///
/// BFS explores the search tree level-by-level, expanding all nodes at depth $d$ before moving to $d + 1$.
///
/// # Complexity
///
/// - **Time Complexity**: $O(b^d)$ where $b$ is the branching factor and $d$ is the goal depth.
/// - **Space Complexity**: $O(b^d)$ to store all nodes at the frontier.
///
/// # Requirements
///
/// - `P::State` must implement `Eq + Hash + Clone` to enable visited state tracking.
/// - Action step costs should be uniform for optimal path length guarantees.
///
/// # Prefer this when
///
/// - All step costs are equal (unweighted transitions).
/// - You need to guarantee finding the solution with the minimum number of steps.
/// - The search space depth is unknown or potentially infinite.
///
/// # Consider instead
///
/// - [`astar`](crate::search::astar()) or [`ucs`](crate::search::ucs()) when transitions have non-uniform costs.
/// - [`dfs`](crate::search::dfs()) or [`iddfs`](crate::search::iddfs()) when memory is severely constrained.
///
/// # References
///
/// - Cormen, T. H., Leiserson, C. E., Rivest, R. L., & Stein, C. (2009). *Introduction to Algorithms* (3rd ed.). MIT Press. Chapter 22.2.
pub fn bfs<P>(problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
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

    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(initial_state.clone());
    frontier.push_back((initial_state, Vec::new(), P::Cost::default()));

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    while let Some((curr_state, moves, cost)) = frontier.pop_front() {
        nodes_expanded += 1;

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
            frontier.push_back((next_state, next_moves, next_cost));
        }
    }

    None
}
