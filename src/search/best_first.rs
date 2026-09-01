//! Greedy Best-First Search.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

use crate::core::problem::SearchProblem;
use crate::core::solution::{SearchMetrics, Solution};

struct Node<State, Move, Cost> {
    state: State,
    moves: Vec<Move>,
    cost: Cost,
    heuristic: Cost,
}

impl<State, Move, Cost: Ord> PartialEq for Node<State, Move, Cost> {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic == other.heuristic
    }
}

impl<State, Move, Cost: Ord> Eq for Node<State, Move, Cost> {}

impl<State, Move, Cost: Ord> Ord for Node<State, Move, Cost> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap ordered purely by heuristic estimate h(n)
        other.heuristic.cmp(&self.heuristic)
    }
}

impl<State, Move, Cost: Ord> PartialOrd for Node<State, Move, Cost> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds a goal state quickly using Greedy Best-First Search.
///
/// Greedy Best-First Search prioritizes nodes with the smallest heuristic estimate $h(n)$
/// to goal, ignoring past path costs.
///
/// # Complexity
///
/// - **Time Complexity**: $O(b^m)$ worst-case, but often significantly faster with good heuristics.
/// - **Space Complexity**: $O(b^m)$ frontier memory.
///
/// # Requirements
///
/// - `P::State` must implement `Eq + Hash + Clone`.
///
/// # Prefer this when
///
/// - Finding *any* solution quickly is more important than optimal path cost.
///
/// # Consider instead
///
/// - [`astar`](crate::search::astar()) when solution path cost optimality is required.
///
/// # References
///
/// - Pearl, J. (1984). *Heuristics: Intelligent Search Strategies for Computer Problem Solving*. Addison-Wesley.
pub fn best_first<P>(problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
where
    P: SearchProblem,
    P::State: Eq + Hash,
{
    let initial_state = problem.initial();
    let initial_h = problem.heuristic(&initial_state);

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

    let mut frontier = BinaryHeap::new();
    let mut visited = HashSet::new();

    visited.insert(initial_state.clone());
    frontier.push(Node {
        state: initial_state,
        moves: Vec::new(),
        cost: P::Cost::default(),
        heuristic: initial_h,
    });

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    while let Some(Node {
        state, moves, cost, ..
    }) = frontier.pop()
    {
        if problem.is_goal(&state) {
            return Some(Solution::new(
                state,
                moves,
                cost,
                SearchMetrics {
                    nodes_expanded,
                    nodes_visited,
                },
            ));
        }

        nodes_expanded += 1;

        for mv in problem.moves(&state) {
            let next_state = problem.apply(&state, &mv);
            if visited.contains(&next_state) {
                continue;
            }

            visited.insert(next_state.clone());
            nodes_visited += 1;

            let step_cost = problem.step_cost(&state, &mv);
            let next_cost = cost + step_cost;
            let next_h = problem.heuristic(&next_state);

            let mut next_moves = moves.clone();
            next_moves.push(mv);

            frontier.push(Node {
                state: next_state,
                moves: next_moves,
                cost: next_cost,
                heuristic: next_h,
            });
        }
    }

    None
}
