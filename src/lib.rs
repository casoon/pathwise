//! `pathwise` — generic search, optimization, and constraint-building blocks for Rust.
//!
//! Concept and scope: see `README.md`. Implementation plan: `plan/`.
//!
//! Module layout follows the layered architecture:
//!
//! ```text
//! core
//!  ├── search
//!  ├── optimization
//!  └── constraint
//! graph
//! ```

pub mod constraint;
pub mod core;
pub mod graph;
pub mod optimization;
pub mod search;

pub use core::problem::{OptimizationProblem, Problem, SearchProblem};
pub use core::solution::{SearchMetrics, Solution};
