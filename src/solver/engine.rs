//! Constraint Solver Engine.

use std::collections::HashMap;
use std::hash::Hash;

use crate::solver::constraint::{Conflict, Constraint};
use crate::solver::domain::Domain;
use crate::solver::variable::VariableId;

/// Advanced Constraint Solver supporting AC propagation and backtracking.
pub struct ConstraintSolver<Val> {
    variables: Vec<VariableId>,
    initial_domains: HashMap<VariableId, Domain<Val>>,
    constraints: Vec<Box<dyn Constraint<Val>>>,
}

impl<Val: Eq + Hash + Clone + Send + Sync> ConstraintSolver<Val> {
    /// Creates a new constraint solver instance.
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            initial_domains: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds a variable with an initial domain of values.
    pub fn add_variable(&mut self, domain: impl IntoIterator<Item = Val>) -> VariableId {
        let var = VariableId(self.variables.len());
        self.variables.push(var);
        self.initial_domains.insert(var, Domain::new(domain));
        var
    }

    /// Adds a constraint to the solver model.
    pub fn add_constraint(&mut self, constraint: impl Constraint<Val> + 'static) {
        self.constraints.push(Box::new(constraint));
    }

    /// Solves the CSP model, returning a complete variable assignment if satisfiable.
    pub fn solve(&self) -> Option<HashMap<VariableId, Val>> {
        let mut domains = self.initial_domains.clone();

        if self.propagate(&mut domains).is_err() {
            return None;
        }

        let mut assignment = HashMap::new();
        if self.backtrack(&mut domains, &mut assignment) {
            Some(assignment)
        } else {
            None
        }
    }

    fn propagate(&self, domains: &mut HashMap<VariableId, Domain<Val>>) -> Result<(), Conflict> {
        let mut changed = true;
        while changed {
            changed = false;
            for c in &self.constraints {
                if c.propagate(domains)? {
                    changed = true;
                }
            }
        }
        Ok(())
    }

    fn backtrack(
        &self,
        domains: &mut HashMap<VariableId, Domain<Val>>,
        assignment: &mut HashMap<VariableId, Val>,
    ) -> bool {
        if assignment.len() == self.variables.len() {
            return true;
        }

        // Select unassigned variable with Minimum Remaining Values (MRV)
        let var = match self
            .variables
            .iter()
            .filter(|v| !assignment.contains_key(v))
            .min_by_key(|v| domains.get(v).map_or(0, |d| d.len()))
        {
            Some(&v) => v,
            None => return true,
        };

        let values: Vec<Val> = domains
            .get(&var)
            .map(|d| d.iter().cloned().collect())
            .unwrap_or_default();

        for val in values {
            let mut local_domains = domains.clone();
            local_domains.insert(var, Domain::new(vec![val.clone()]));

            if self.propagate(&mut local_domains).is_ok() {
                assignment.insert(var, val);
                if self.backtrack(&mut local_domains, assignment) {
                    return true;
                }
                assignment.remove(&var);
            }
        }

        false
    }
}

impl<Val: Eq + Hash + Clone + Send + Sync> Default for ConstraintSolver<Val> {
    fn default() -> Self {
        Self::new()
    }
}
