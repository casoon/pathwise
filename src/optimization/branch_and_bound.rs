//! Branch and Bound Exact Optimization.

use crate::core::problem::Problem;
use crate::core::solution::{SearchMetrics, Solution};
use crate::optimization::coordination::Coordination;

/// Trait for problems solved via Branch and Bound optimization.
pub trait BranchAndBoundProblem: Problem {
    /// Objective score / cost type.
    type Score: Ord + Copy;

    /// Evaluates exact score of a complete state solution.
    fn score(&self, state: &Self::State) -> Self::Score;

    /// Computes an optimistic upper bound on achievable score from a partial `state`.
    ///
    /// For maximization problems, bound $\ge$ optimal reachable score in subtree.
    fn upper_bound(&self, state: &Self::State) -> Self::Score;
}

/// Finds an exact optimal solution using Branch and Bound search.
///
/// Branch and Bound prunes subtrees whose optimistic upper bound estimate $\le$ best solution score found so far.
///
/// # Complexity
///
/// - **Time Complexity**: $O(b^m)$ worst case, significantly pruned with tight bounding functions.
/// - **Space Complexity**: $O(b \cdot m)$ depth-first search stack space.
///
/// # Prefer this when
///
/// - Exact global optimum is required for combinatorial optimization problems (e.g. Knapsack, TSP).
/// - Tight bounding functions exist.
///
/// # References
///
/// - Land, A. H., & Doig, A. G. (1960). An automatic method of solving discrete programming problems. *Econometrica*, 28(3), 497-520.
///
/// If `coordination.cancellation` is set (see [`crate::core::cancellation::CancellationToken`]),
/// the search checks it once per stack pop and returns the best solution found so far (possibly
/// `None`) instead of exhausting the search space. If `coordination.incumbent` is set (see
/// [`crate::core::incumbent::SharedIncumbent`]), improving solutions are offered to it, and a
/// stronger incumbent found by another portfolio worker is adopted as this search's own bound
/// before pruning — so this call both benefits from, and contributes to, coordinated portfolio
/// search.
pub fn branch_and_bound<P>(
    problem: &P,
    coordination: &Coordination<P::State, P::Score>,
) -> Option<Solution<P::State, P::Move, P::Score>>
where
    P: BranchAndBoundProblem,
{
    let initial_state = problem.initial();
    let initial_bound = problem.upper_bound(&initial_state);

    let mut stack = vec![(initial_state, Vec::new(), initial_bound)];
    let mut best_solution: Option<Solution<P::State, P::Move, P::Score>> = None;

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    while let Some((curr_state, moves, bound)) = stack.pop() {
        if coordination
            .cancellation
            .as_ref()
            .is_some_and(|token| token.is_cancelled())
        {
            break;
        }

        // Adopt a better portfolio-wide incumbent before bounding: some other worker (e.g.
        // Local Search, LNS) may have found a stronger solution than this subtree knows about
        // yet.
        if let Some(incumbent) = &coordination.incumbent
            && let Some((shared_state, shared_score)) = incumbent.best()
            && best_solution
                .as_ref()
                .is_none_or(|best| shared_score > best.cost)
        {
            best_solution = Some(Solution::new(
                shared_state,
                Vec::new(),
                shared_score,
                SearchMetrics {
                    nodes_expanded,
                    nodes_visited,
                },
            ));
        }

        if best_solution
            .as_ref()
            .is_some_and(|best| bound <= best.cost)
        {
            continue;
        }

        if problem.is_goal(&curr_state) {
            let score = problem.score(&curr_state);
            let is_better = match best_solution {
                Some(ref best) => score > best.cost,
                None => true,
            };

            if is_better {
                if let Some(incumbent) = &coordination.incumbent {
                    incumbent.offer(&curr_state, score);
                }
                best_solution = Some(Solution::new(
                    curr_state,
                    moves,
                    score,
                    SearchMetrics {
                        nodes_expanded,
                        nodes_visited,
                    },
                ));
            }
            continue;
        }

        nodes_expanded += 1;

        for mv in problem.moves(&curr_state) {
            let next_state = problem.apply(&curr_state, &mv);
            let next_bound = problem.upper_bound(&next_state);

            if best_solution
                .as_ref()
                .is_some_and(|best| next_bound <= best.cost)
            {
                continue;
            }

            nodes_visited += 1;
            let mut next_moves = moves.clone();
            next_moves.push(mv);

            stack.push((next_state, next_moves, next_bound));
        }
    }

    if let Some(mut sol) = best_solution {
        sol.metrics = SearchMetrics {
            nodes_expanded,
            nodes_visited,
        };
        Some(sol)
    } else {
        None
    }
}
