//! Beam Search Algorithm.

use std::hash::Hash;

use crate::core::problem::SearchProblem;
use crate::core::solution::{SearchMetrics, Solution};

/// Options for Beam Search execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeamSearchOptions {
    /// Maximum number of states retained at each level (beam width $W$).
    pub beam_width: usize,
    /// Maximum search depth limit.
    pub max_depth: Option<usize>,
}

impl BeamSearchOptions {
    /// Creates a new `BeamSearchOptions` with specified beam width.
    pub fn new(beam_width: usize) -> Self {
        Self {
            beam_width,
            max_depth: None,
        }
    }
}

/// Performs a heuristic-guided, memory-bounded Beam Search.
///
/// Beam Search retains only the $W$ best candidates (ranked by heuristic cost $h(n)$) at each depth level.
///
/// # Complexity
///
/// - **Time Complexity**: $O(d \cdot W \cdot b)$ where $d$ is depth, $W$ is beam width, and $b$ is branching factor.
/// - **Space Complexity**: $O(W \cdot b)$ bounded memory.
///
/// # Requirements
///
/// - Beam width $W > 0$.
/// - `P::State` must implement `Eq + Hash + Clone`.
///
/// # Prefer this when
///
/// - The search space is extremely large or intractable for exact algorithms like A*.
/// - Memory footprint must stay strictly bounded.
///
/// # Consider instead
///
/// - [`astar`](crate::search::astar()) when completeness and exact solution optimality are required.
///
/// # References
///
/// - Lowerre, B. T. (1976). *The HARPY Speech Recognition System*. Carnegie-Mellon University.
pub fn beam_search<P>(
    problem: &P,
    options: BeamSearchOptions,
) -> Option<Solution<P::State, P::Move, P::Cost>>
where
    P: SearchProblem,
    P::State: Eq + Hash,
{
    let beam_width = options.beam_width;
    if beam_width == 0 {
        return None;
    }

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

    struct Candidate<State, Move, Cost> {
        state: State,
        moves: Vec<Move>,
        cost: Cost,
        heuristic: Cost,
    }

    let mut current_level = vec![Candidate {
        state: initial_state,
        moves: Vec::new(),
        cost: P::Cost::default(),
        heuristic: initial_h,
    }];

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;
    let mut depth = 0;

    while !current_level.is_empty() {
        if options.max_depth.is_some_and(|max| depth >= max) {
            break;
        }

        let mut next_candidates = Vec::new();

        for candidate in current_level {
            nodes_expanded += 1;

            for mv in problem.moves(&candidate.state) {
                let next_state = problem.apply(&candidate.state, &mv);
                nodes_visited += 1;

                let step_cost = problem.step_cost(&candidate.state, &mv);
                let next_cost = candidate.cost + step_cost;
                let next_h = problem.heuristic(&next_state);

                let mut next_moves = candidate.moves.clone();
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

                next_candidates.push(Candidate {
                    state: next_state,
                    moves: next_moves,
                    cost: next_cost,
                    heuristic: next_h,
                });
            }
        }

        // Sort candidates by heuristic cost h(n) ascending and truncate to beam_width
        next_candidates.sort_by_key(|a| a.heuristic);

        next_candidates.truncate(beam_width);

        current_level = next_candidates;
        depth += 1;
    }

    None
}
