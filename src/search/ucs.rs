//! Uniform Cost Search (Dijkstra's Algorithm).

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

use crate::core::problem::SearchProblem;
use crate::core::solution::{SearchMetrics, Solution};

struct Node<State, Move, Cost> {
    state: State,
    moves: Vec<Move>,
    cost: Cost,
}

impl<State, Move, Cost: Ord> PartialEq for Node<State, Move, Cost> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<State, Move, Cost: Ord> Eq for Node<State, Move, Cost> {}

impl<State, Move, Cost: Ord> Ord for Node<State, Move, Cost> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap ordering in std::collections::BinaryHeap
        other.cost.cmp(&self.cost)
    }
}

impl<State, Move, Cost: Ord> PartialOrd for Node<State, Move, Cost> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds the lowest-cost path using Uniform Cost Search (Dijkstra's algorithm).
///
/// UCS expands nodes in order of their accumulated path cost $g(n)$.
///
/// # Complexity
///
/// - **Time Complexity**: $O((V + E) \log V)$ where $V$ is state space size and $E$ is transitions.
/// - **Space Complexity**: $O(V)$ to maintain frontier heap and best-cost table.
///
/// # Requirements
///
/// - Step costs must be non-negative ($g(n)$ monotonically increases).
/// - `P::State` must implement `Eq + Hash + Clone`.
///
/// # Prefer this when
///
/// - Transitions have non-uniform non-negative edge costs.
/// - No heuristic is available or admissible.
///
/// # Consider instead
///
/// - [`astar`](crate::search::astar()) when an admissible heuristic is available.
/// - [`bfs`](crate::search::bfs()) when all step costs are equal.
///
/// # References
///
/// - Dijkstra, E. W. (1959). A note on two problems in connexion with graphs. *Numerische Mathematik*, 1(1), 269-271.
pub fn ucs<P>(problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
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

    let mut frontier = BinaryHeap::new();
    let mut best_costs = HashMap::new();

    best_costs.insert(initial_state.clone(), P::Cost::default());
    frontier.push(Node {
        state: initial_state,
        moves: Vec::new(),
        cost: P::Cost::default(),
    });

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    while let Some(Node { state, moves, cost }) = frontier.pop() {
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

        if best_costs.get(&state).is_some_and(|&best| cost > best) {
            continue;
        }

        nodes_expanded += 1;

        for mv in problem.moves(&state) {
            let next_state = problem.apply(&state, &mv);
            let step_cost = problem.step_cost(&state, &mv);
            let next_cost = cost + step_cost;

            let is_better = match best_costs.get(&next_state) {
                Some(&best) => next_cost < best,
                None => true,
            };

            if is_better {
                best_costs.insert(next_state.clone(), next_cost);
                nodes_visited += 1;

                let mut next_moves = moves.clone();
                next_moves.push(mv);

                frontier.push(Node {
                    state: next_state,
                    moves: next_moves,
                    cost: next_cost,
                });
            }
        }
    }

    None
}
