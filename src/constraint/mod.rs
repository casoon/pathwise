//! Constraint Satisfaction Problem (CSP) solving module.
//!
//! Includes CSP problem representation, Backtracking search, AC-3 constraint propagation, and MRV / LCV heuristics.

pub mod ac3;
pub mod backtracking;
pub mod heuristics;
pub mod problem;

pub use ac3::ac3;
pub use backtracking::{BacktrackingOptions, backtracking};
pub use heuristics::{lcv_value_sorter, mrv_variable_selector};
pub use problem::{BinaryConstraint, Csp};
