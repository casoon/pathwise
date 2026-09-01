//! High-level Constraint Solver Framework.
//!
//! Provides finite domain variable management, constraint propagation traits, and a complete CSP solver engine.

pub mod constraint;
pub mod domain;
pub mod engine;
pub mod variable;

pub use constraint::{AllDifferent, Conflict, Constraint};
pub use domain::Domain;
pub use engine::ConstraintSolver;
pub use variable::VariableId;
