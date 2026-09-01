//! Variable and value ordering heuristics for CSP (MRV, LCV).

use std::collections::HashMap;
use std::hash::Hash;

use crate::constraint::problem::Csp;

/// Minimum Remaining Values (MRV / Fail-First) variable ordering heuristic.
///
/// Selects the unassigned variable with the fewest legal remaining values in its domain.
pub fn mrv_variable_selector<Var, Val>(
    csp: &Csp<Var, Val>,

    assignment: &HashMap<Var, Val>,
    domains: &HashMap<Var, Vec<Val>>,
) -> Option<Var>
where
    Var: Eq + Hash + Clone,
    Val: Eq + Hash + Clone,
{
    let mut min_var = None;
    let mut min_count = usize::MAX;

    for var in &csp.variables {
        if assignment.contains_key(var) {
            continue;
        }

        let count = domains.get(var).map_or(0, |d| d.len());
        if count < min_count {
            min_count = count;
            min_var = Some(var.clone());
        }
    }

    min_var
}

/// Least Constraining Value (LCV) value ordering heuristic.
///
/// Sorts domain values for `var` such that values ruling out the fewest remaining choices for neighboring variables come first.
pub fn lcv_value_sorter<Var, Val>(
    csp: &Csp<Var, Val>,
    var: &Var,
    assignment: &HashMap<Var, Val>,
    domains: &HashMap<Var, Vec<Val>>,
) -> Vec<Val>
where
    Var: Eq + Hash + Clone,
    Val: Eq + Hash + Clone,
{
    let values = match domains.get(var) {
        Some(v) => v.clone(),
        None => return Vec::new(),
    };

    let mut value_scores: Vec<(Val, usize)> = values
        .into_iter()
        .map(|val| {
            let mut constrained_count = 0;

            for neighbor in &csp.variables {
                if neighbor == var || assignment.contains_key(neighbor) {
                    continue;
                }

                if let Some(neighbor_domain) = domains.get(neighbor) {
                    for n_val in neighbor_domain {
                        if !csp.is_consistent_pair(var, &val, neighbor, n_val) {
                            constrained_count += 1;
                        }
                    }
                }
            }

            (val, constrained_count)
        })
        .collect();

    // Sort ascending by constrained choices (least constraining value first)
    value_scores.sort_by_key(|(_, score)| *score);
    value_scores.into_iter().map(|(val, _)| val).collect()
}
