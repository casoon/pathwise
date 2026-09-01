//! Core abstractions: `Problem`, `SearchProblem`, `OptimizationProblem`, `Solution`, `SearchStrategy`, `TerminationCondition`, `CancellationToken`, and `SharedIncumbent`.

pub mod cancellation;
pub mod incumbent;
pub mod problem;
pub mod solution;
pub mod strategy;
pub mod termination;

pub use cancellation::CancellationToken;
pub use incumbent::SharedIncumbent;
pub use problem::{OptimizationProblem, Problem, SearchProblem};
pub use solution::{SearchMetrics, Solution};
pub use strategy::{
    AStarStrategy, BeamSearchStrategy, BestFirstStrategy, BfsStrategy, DfsStrategy, IddfsStrategy,
    SearchStrategy, UcsStrategy, solve,
};
pub use termination::TerminationCondition;
