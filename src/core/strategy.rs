//! Search Strategy Pattern Abstraction.

use std::hash::Hash;

use crate::core::problem::SearchProblem;
use crate::core::solution::Solution;
use crate::search::{
    BeamSearchOptions, DfsOptions, astar, beam_search, best_first, bfs, dfs, iddfs, ucs,
};

/// Common trait for interchangeable search strategies.
pub trait SearchStrategy<P: SearchProblem> {
    /// Executes the search strategy on `problem`.
    fn solve(&self, problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
    where
        P::State: Eq + Hash;
}

/// A* Search strategy wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AStarStrategy;

impl<P: SearchProblem> SearchStrategy<P> for AStarStrategy {
    fn solve(&self, problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
    where
        P::State: Eq + Hash,
    {
        astar(problem)
    }
}

/// Breadth-First Search strategy wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BfsStrategy;

impl<P: SearchProblem> SearchStrategy<P> for BfsStrategy {
    fn solve(&self, problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
    where
        P::State: Eq + Hash,
    {
        bfs(problem)
    }
}

/// Depth-First Search strategy wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DfsStrategy {
    /// Depth search options.
    pub options: DfsOptions,
}

impl<P: SearchProblem> SearchStrategy<P> for DfsStrategy {
    fn solve(&self, problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
    where
        P::State: Eq + Hash,
    {
        dfs(problem, self.options)
    }
}

/// Uniform Cost Search (Dijkstra) strategy wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UcsStrategy;

impl<P: SearchProblem> SearchStrategy<P> for UcsStrategy {
    fn solve(&self, problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
    where
        P::State: Eq + Hash,
    {
        ucs(problem)
    }
}

/// Greedy Best-First Search strategy wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BestFirstStrategy;

impl<P: SearchProblem> SearchStrategy<P> for BestFirstStrategy {
    fn solve(&self, problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
    where
        P::State: Eq + Hash,
    {
        best_first(problem)
    }
}

/// Beam Search strategy wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeamSearchStrategy {
    /// Beam search options.
    pub options: BeamSearchOptions,
}

impl BeamSearchStrategy {
    /// Creates a new `BeamSearchStrategy` with specified beam width.
    pub fn new(beam_width: usize) -> Self {
        Self {
            options: BeamSearchOptions::new(beam_width),
        }
    }
}

impl<P: SearchProblem> SearchStrategy<P> for BeamSearchStrategy {
    fn solve(&self, problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
    where
        P::State: Eq + Hash,
    {
        beam_search(problem, self.options)
    }
}

/// Iterative Deepening DFS strategy wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IddfsStrategy {
    /// Maximum depth limit.
    pub max_depth_limit: usize,
}

impl IddfsStrategy {
    /// Creates a new `IddfsStrategy` with specified max depth limit.
    pub fn new(max_depth_limit: usize) -> Self {
        Self { max_depth_limit }
    }
}

impl<P: SearchProblem> SearchStrategy<P> for IddfsStrategy {
    fn solve(&self, problem: &P) -> Option<Solution<P::State, P::Move, P::Cost>>
    where
        P::State: Eq + Hash,
    {
        iddfs(problem, self.max_depth_limit)
    }
}

/// Convenience function solving `problem` using `strategy`.
pub fn solve<P, S>(problem: &P, strategy: S) -> Option<Solution<P::State, P::Move, P::Cost>>
where
    P: SearchProblem,
    P::State: Eq + Hash,
    S: SearchStrategy<P>,
{
    strategy.solve(problem)
}
