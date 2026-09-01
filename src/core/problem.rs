//! Core problem abstractions and trait definitions for search and optimization.

use std::ops::Add;

/// The foundational trait representing a discrete state space problem.
///
/// A `Problem` defines state representation, valid move generation, state transitions,
/// and goal state evaluation.
pub trait Problem {
    /// The state type representing a configuration or node in the search space.
    type State: Clone;

    /// The move or transition action type.
    type Move: Clone;

    /// Returns the starting state of the problem.
    fn initial(&self) -> Self::State;

    /// Returns an iterator over valid moves executable from `state`.
    fn moves(&self, state: &Self::State) -> impl Iterator<Item = Self::Move>;

    /// Applies `mv` to `state` and returns the resulting state.
    fn apply(&self, state: &Self::State, mv: &Self::Move) -> Self::State;

    /// Determines whether `state` satisfies the goal condition.
    fn is_goal(&self, state: &Self::State) -> bool;
}

/// A search problem supporting path costs and heuristic evaluation.
///
/// Implement this trait for graph search algorithms such as A*, Uniform Cost Search (Dijkstra),
/// Greedy Best-First Search, BFS, DFS, and Beam Search.
pub trait SearchProblem: Problem {
    /// Numeric cost type representing path costs and heuristic estimates.
    type Cost: Copy + Ord + Add<Output = Self::Cost> + Default;

    /// Returns the step cost of applying `mv` from `state`.
    fn step_cost(&self, state: &Self::State, mv: &Self::Move) -> Self::Cost;

    /// Returns a heuristic cost estimate from `state` to a goal.
    ///
    /// To guarantee optimality in A*, the heuristic function must be *admissible*
    /// (it never overestimates the actual cost to reach a goal).
    fn heuristic(&self, state: &Self::State) -> Self::Cost {
        let _ = state;
        Self::Cost::default()
    }
}

/// An optimization problem evaluated by an objective score.
///
/// Implement this trait for optimization algorithms like Branch & Bound, Hill Climbing,
/// Local Search, and Simulated Annealing.
pub trait OptimizationProblem: Problem {
    /// Score type used to evaluate states.
    type Score: Ord + Copy;

    /// Evaluates the objective value or quality score of `state`.
    fn score(&self, state: &Self::State) -> Self::Score;
}
