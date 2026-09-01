//! Arc Consistency (AC-3) Algorithm.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use crate::constraint::problem::Csp;

/// Enforces Arc Consistency (AC-3) on a CSP problem, pruning inconsistent domain values.
///
/// AC-3 systematically inspects arcs $(X_i, X_j)$ and removes values $v \in D_i$ that have no consistent match in $D_j$.
///
/// # Complexity
///
/// - **Time Complexity**: $O(c \cdot d^3)$ where $c$ is number of binary constraints and $d$ is max domain size.
/// - **Space Complexity**: $O(c)$ for the arc queue.
///
/// # Returns
///
/// Returns `true` if arc consistency was established with non-empty domains, or `false` if a domain became empty (no solution possible).
///
/// # References
///
/// - Mackworth, A. K. (1977). Consistency in networks of relations. *Artificial Intelligence*, 8(1), 99-118.
pub fn ac3<Var, Val>(csp: &Csp<Var, Val>, domains: &mut HashMap<Var, Vec<Val>>) -> bool
where
    Var: Eq + Hash + Clone,
    Val: Eq + Hash + Clone,
{
    let mut queue = VecDeque::new();

    // Populate queue with all constraint arcs
    for (var1, var2) in csp.constraints.keys() {
        queue.push_back((var1.clone(), var2.clone()));
    }

    while let Some((xi, xj)) = queue.pop_front() {
        if revise(csp, domains, &xi, &xj) {
            let domain_len = domains.get(&xi).map_or(0, |d| d.len());
            if domain_len == 0 {
                return false; // Inconsistency detected
            }

            // Re-add arcs (Xk, Xi) for all neighbors Xk of Xi (except Xj)
            for (var1, var2) in csp.constraints.keys() {
                if var2 == &xi && var1 != &xj {
                    queue.push_back((var1.clone(), xi.clone()));
                } else if var1 == &xi && var2 != &xj {
                    queue.push_back((var2.clone(), xi.clone()));
                }
            }
        }
    }

    true
}

/// Helper function `REVISE(CSP, Xi, Xj)`: removes values from $D_i$ that have no support in $D_j$.
fn revise<Var, Val>(
    csp: &Csp<Var, Val>,
    domains: &mut HashMap<Var, Vec<Val>>,
    xi: &Var,
    xj: &Var,
) -> bool
where
    Var: Eq + Hash + Clone,
    Val: Eq + Hash + Clone,
{
    let mut revised = false;

    let xj_domain = match domains.get(xj) {
        Some(d) => d.clone(),
        None => return false,
    };

    if let Some(xi_domain) = domains.get_mut(xi) {
        let mut new_domain = Vec::new();

        for vi in xi_domain.drain(..) {
            let has_support = xj_domain
                .iter()
                .any(|vj| csp.is_consistent_pair(xi, &vi, xj, vj));

            if has_support {
                new_domain.push(vi);
            } else {
                revised = true;
            }
        }

        *xi_domain = new_domain;
    }

    revised
}
