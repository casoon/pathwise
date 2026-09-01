//! A* Search Algorithm.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

use crate::core::problem::SearchProblem;
use crate::core::solution::{SearchMetrics, Solution};

struct AStarNode<State, Move, Cost> {
    state: State,
    moves: Vec<Move>,
    g_cost: Cost,
    f_cost: Cost,
}

impl<State, Move, Cost: Ord> PartialEq for AStarNode<State, Move, Cost> {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost == other.f_cost
    }
}

impl<State, Move, Cost: Ord> Eq for AStarNode<State, Move, Cost> {}

impl<State, Move, Cost: Ord> Ord for AStarNode<State, Move, Cost> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap ordering based on f(n) = g(n) + h(n)
        other.f_cost.cmp(&self.f_cost)
    }
}

impl<State, Move, Cost: Ord> PartialOrd for AStarNode<State, Move, Cost> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds the optimal shortest path using the A* Search algorithm.
///
/// A* evaluates nodes using $f(n) = g(n) + h(n)$, where $g(n)$ is the path cost from initial state
/// to node $n$, and $h(n)$ is the heuristic estimate of cost from $n$ to goal.
///
/// # Complexity
///
/// - **Time Complexity**: $O(b^d)$ in worst case, sub-exponential with accurate heuristic $h(n)$.
/// - **Space Complexity**: $O(b^d)$ to store open and closed sets in memory.
///
/// # Requirements
///
/// - The heuristic $h(n)$ must be **admissible** (never overestimates remaining cost) to guarantee optimality.
/// - For graph search without reopenings, $h(n)$ should also be **consistent** / **monotone** ($h(n) \le c(n, a, n') + h(n')$).
/// - `P::State` must implement `Eq + Hash + Clone`.
///
/// # Prefer this when
///
/// - An informative admissible heuristic exists for the domain.
/// - The optimal lowest-cost solution is required.
///
/// # Consider instead
///
/// - [`ucs`](crate::search::ucs()) when no heuristic function is available.
/// - [`bfs`](crate::search::bfs()) for unweighted graphs with uniform edge costs.
/// - [`beam_search`](crate::search::beam_search()) when memory footprint must be tightly capped.
///
/// # References
///
/// - Hart, P. E., Nilsson, N. J., & Raphael, B. (1968). A formal basis for the heuristic determination of minimum cost paths. *IEEE Transactions on Systems Science and Cybernetics*, 4(2), 100-107.
pub fn astar<P>(problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
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
    let mut best_g = HashMap::new();

    best_g.insert(initial_state.clone(), P::Cost::default());
    frontier.push(AStarNode {
        state: initial_state,
        moves: Vec::new(),
        g_cost: P::Cost::default(),
        f_cost: initial_h,
    });

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    while let Some(AStarNode {
        state,
        moves,
        g_cost,
        ..
    }) = frontier.pop()
    {
        if problem.is_goal(&state) {
            return Some(Solution::new(
                state,
                moves,
                g_cost,
                SearchMetrics {
                    nodes_expanded,
                    nodes_visited,
                },
            ));
        }

        if best_g.get(&state).is_some_and(|&best| g_cost > best) {
            continue;
        }

        nodes_expanded += 1;

        for mv in problem.moves(&state) {
            let next_state = problem.apply(&state, &mv);
            let step_cost = problem.step_cost(&state, &mv);
            let next_g = g_cost + step_cost;

            let is_better = match best_g.get(&next_state) {
                Some(&prev_g) => next_g < prev_g,
                None => true,
            };

            if is_better {
                best_g.insert(next_state.clone(), next_g);
                nodes_visited += 1;

                let next_h = problem.heuristic(&next_state);
                let next_f = next_g + next_h;

                let mut next_moves = moves.clone();
                next_moves.push(mv);

                frontier.push(AStarNode {
                    state: next_state,
                    moves: next_moves,
                    g_cost: next_g,
                    f_cost: next_f,
                });
            }
        }
    }

    None
}
