//! Optimization algorithms module.
//!
//! Includes Branch & Bound exact optimization, Hill Climbing, Local Search, Simulated Annealing, Tabu Search, Large Neighborhood Search, and budgeted local search for expensive, asynchronous evaluations.

pub mod branch_and_bound;
pub mod budgeted;
pub mod coordination;
pub mod hill_climbing;
pub mod lns;
pub mod local_search;
pub mod simulated_annealing;
pub mod tabu_search;

pub use branch_and_bound::{BranchAndBoundProblem, branch_and_bound};
pub use budgeted::{
    AsyncOptimizationProblem, BudgetedSearchOptions, BudgetedSolution, Evaluation,
    budgeted_local_search,
};
pub use coordination::Coordination;
pub use hill_climbing::{HillClimbingOptions, hill_climbing};
pub use lns::{LnsOptions, LnsProblem, large_neighborhood_search};
pub use local_search::{LocalSearchOptions, local_search};
pub use simulated_annealing::{SimulatedAnnealingOptions, simulated_annealing};
pub use tabu_search::{TabuSearchOptions, tabu_search};
