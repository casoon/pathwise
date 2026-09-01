//! `pathwise` — generic search, optimization, constraint-solving, and scheduling building blocks for Rust.
//!
//! Concept and scope: see `README.md`. Implementation plan: `plan/`.
//!
//! Module layout follows the layered architecture:
//!
//! ```text
//! core
//!  ├── search
//!  ├── optimization
//!  ├── constraint
//!  └── solver
//! graph
//! scheduling
//! ```

pub mod constraint;
pub mod core;
pub mod graph;
pub mod optimization;
pub mod scheduling;
pub mod search;
pub mod solver;

pub use core::problem::{OptimizationProblem, Problem, SearchProblem};
pub use core::solution::{SearchMetrics, Solution};
pub use scheduling::{Activity, Interval, Lesson, ScheduleAssignment, TimetableProblem};
pub use solver::{ConstraintSolver, Domain, VariableId};
