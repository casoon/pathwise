//! Constraint traits and domain propagation contracts.

use std::collections::HashMap;
use std::hash::Hash;

use crate::solver::domain::Domain;
use crate::solver::variable::VariableId;

/// Error indicating a domain became empty during constraint propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conflict(pub VariableId);

/// Trait for generalized constraints on variable domains.
pub trait Constraint<Val: Eq + Hash + Clone + Send + Sync>: Send + Sync {
    /// Returns the variables involved in this constraint.
    fn variables(&self) -> Vec<VariableId>;

    /// Propagates constraint restriction across variable domains.
    /// Returns `Ok(true)` if domains changed, `Ok(false)` if unchanged, or `Err(Conflict)` if inconsistent.
    fn propagate(&self, domains: &mut HashMap<VariableId, Domain<Val>>) -> Result<bool, Conflict>;
}

/// Binary AllDifferent constraint for a set of variables.
pub struct AllDifferent<Val> {
    vars: Vec<VariableId>,
    _phantom: std::marker::PhantomData<Val>,
}

impl<Val: Eq + Hash + Clone + Send + Sync> AllDifferent<Val> {
    /// Creates an `AllDifferent` constraint for `vars`.
    pub fn new(vars: Vec<VariableId>) -> Self {
        Self {
            vars,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Val: Eq + Hash + Clone + Send + Sync> Constraint<Val> for AllDifferent<Val> {
    fn variables(&self) -> Vec<VariableId> {
        self.vars.clone()
    }

    fn propagate(&self, domains: &mut HashMap<VariableId, Domain<Val>>) -> Result<bool, Conflict> {
        let mut changed = false;

        // Propagate singleton assignments (if variable A is fixed to val, no other variable can take val)
        let singletons: Vec<(VariableId, Val)> = self
            .vars
            .iter()
            .filter_map(|&v| {
                domains
                    .get(&v)
                    .and_then(|d| d.singleton())
                    .map(|val| (v, val))
            })
            .collect();

        for (fixed_var, fixed_val) in singletons {
            for &other_var in &self.vars {
                if other_var == fixed_var {
                    continue;
                }
                if let Some(other_domain) = domains.get_mut(&other_var)
                    && other_domain.remove(&fixed_val)
                {
                    changed = true;

                    if other_domain.is_empty() {
                        return Err(Conflict(other_var));
                    }
                }
            }
        }

        Ok(changed)
    }
}
