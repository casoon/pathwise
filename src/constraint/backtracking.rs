//! Constraint Backtracking Search.

use std::collections::HashMap;
use std::hash::Hash;

use crate::constraint::ac3::ac3;
use crate::constraint::heuristics::{lcv_value_sorter, mrv_variable_selector};
use crate::constraint::problem::Csp;

/// Options configuring CSP Backtracking Search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BacktrackingOptions {
    /// Enable Minimum Remaining Values (MRV) variable selection heuristic.
    pub use_mrv: bool,
    /// Enable Least Constraining Value (LCV) value ordering heuristic.
    pub use_lcv: bool,
    /// Enable AC-3 inference propagation during search.
    pub use_ac3: bool,
}

/// Solves a CSP using Backtracking search with optional heuristics and AC-3 inference.
///
/// # Complexity
///
/// - **Time Complexity**: $O(d^n)$ worst case where $d$ is max domain size and $n$ is variable count.
/// - **Space Complexity**: $O(n \cdot d)$ stack memory.
///
/// # Returns
///
/// Returns `Some(HashMap<Var, Val>)` mapping variables to consistent values, or `None` if unsatisfiable.
///
/// # References
///
/// - Dechter, R. (2003). *Constraint Processing*. Morgan Kaufmann.
pub fn backtracking<Var, Val>(
    csp: &Csp<Var, Val>,
    options: BacktrackingOptions,
) -> Option<HashMap<Var, Val>>
where
    Var: Eq + Hash + Clone,
    Val: Eq + Hash + Clone,
{
    let mut domains = csp.domains.clone();
    if options.use_ac3 && !ac3(csp, &mut domains) {
        return None;
    }

    let mut assignment = HashMap::new();
    if backtrack_search(csp, &mut assignment, &mut domains, options) {
        Some(assignment)
    } else {
        None
    }
}

fn backtrack_search<Var, Val>(
    csp: &Csp<Var, Val>,
    assignment: &mut HashMap<Var, Val>,
    domains: &mut HashMap<Var, Vec<Val>>,
    options: BacktrackingOptions,
) -> bool
where
    Var: Eq + Hash + Clone,
    Val: Eq + Hash + Clone,
{
    if assignment.len() == csp.variables.len() {
        return true; // All variables assigned
    }

    let var = if options.use_mrv {
        match mrv_variable_selector(csp, assignment, domains) {
            Some(v) => v,
            None => return false,
        }
    } else {
        // Simple first unassigned variable
        match csp.variables.iter().find(|v| !assignment.contains_key(*v)) {
            Some(v) => v.clone(),
            None => return true,
        }
    };

    let values = if options.use_lcv {
        lcv_value_sorter(csp, &var, assignment, domains)
    } else {
        domains.get(&var).cloned().unwrap_or_default()
    };

    for val in values {
        if csp.is_consistent_assignment(assignment, &var, &val) {
            assignment.insert(var.clone(), val.clone());

            let mut local_domains = domains.clone();
            local_domains.insert(var.clone(), vec![val.clone()]);

            let is_viable = if options.use_ac3 {
                ac3(csp, &mut local_domains)
            } else {
                true
            };

            if is_viable && backtrack_search(csp, assignment, &mut local_domains, options) {
                return true;
            }

            assignment.remove(&var);
        }
    }

    false
}
