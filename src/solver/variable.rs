//! Variable representation for high-level constraint solver.

use std::fmt::Display;
use std::hash::Hash;

/// Strongly typed CSP variable identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VariableId(pub usize);

impl Display for VariableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Var({})", self.0)
    }
}
