//! Constraint Satisfaction Problem (CSP) representation.

use std::collections::HashMap;

use std::hash::Hash;

/// A binary constraint function checking if assignment `(val1, val2)` for `(var1, var2)` is consistent.
pub type BinaryConstraint<Var, Val> = Box<dyn Fn(&Var, &Val, &Var, &Val) -> bool + Send + Sync>;

/// Represents a Constraint Satisfaction Problem (CSP).
pub struct Csp<Var, Val>
where
    Var: Eq + Hash + Clone,
    Val: Eq + Hash + Clone,
{
    /// Variables in the CSP.
    pub variables: Vec<Var>,
    /// Domain of possible values for each variable.
    pub domains: HashMap<Var, Vec<Val>>,
    /// Binary constraints specified as key `(Var1, Var2)` mapping to list of constraint predicates.
    pub constraints: HashMap<(Var, Var), Vec<BinaryConstraint<Var, Val>>>,
}

impl<Var, Val> Csp<Var, Val>
where
    Var: Eq + Hash + Clone,
    Val: Eq + Hash + Clone,
{
    /// Creates a new empty CSP.
    pub fn new(variables: Vec<Var>, domains: HashMap<Var, Vec<Val>>) -> Self {
        Self {
            variables,
            domains,
            constraints: HashMap::new(),
        }
    }

    /// Adds a directional or symmetric binary constraint between `var1` and `var2`.
    pub fn add_constraint<F>(&mut self, var1: Var, var2: Var, constraint: F)
    where
        F: Fn(&Var, &Val, &Var, &Val) -> bool + Send + Sync + 'static,
    {
        self.constraints
            .entry((var1, var2))
            .or_default()
            .push(Box::new(constraint));
    }

    /// Checks if assigning `val1` to `var1` and `val2` to `var2` satisfies all registered constraints between them.
    pub fn is_consistent_pair(&self, var1: &Var, val1: &Val, var2: &Var, val2: &Val) -> bool {
        if let Some(list) = self.constraints.get(&(var1.clone(), var2.clone())) {
            for c in list {
                if !c(var1, val1, var2, val2) {
                    return false;
                }
            }
        }
        if let Some(list) = self.constraints.get(&(var2.clone(), var1.clone())) {
            for c in list {
                if !c(var2, val2, var1, val1) {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if partial assignment `assignment` remains consistent when adding `(var, val)`.
    pub fn is_consistent_assignment(
        &self,
        assignment: &HashMap<Var, Val>,
        var: &Var,
        val: &Val,
    ) -> bool {
        for (assigned_var, assigned_val) in assignment {
            if assigned_var == var {
                continue;
            }
            if !self.is_consistent_pair(var, val, assigned_var, assigned_val) {
                return false;
            }
        }
        true
    }
}
