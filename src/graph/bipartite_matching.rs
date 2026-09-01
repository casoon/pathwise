//! Maximum Bipartite Matching (Hopcroft-Karp Algorithm).

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Computes a Maximum Bipartite Matching on a bipartite graph using the Hopcroft-Karp algorithm.
///
/// Finds the maximum cardinality set of edges such that no two edges share a common vertex.
///
/// # Complexity
///
/// - **Time Complexity**: $O(E \sqrt{V})$ where $V = |U| + |V|$ and $E$ is edge count.
/// - **Space Complexity**: $O(V + E)$ auxiliary memory.
///
/// # Returns
///
/// Returns `Vec<(U, V)>` containing matching pairs between left set $U$ and right set $V$.
///
/// # References
///
/// - Hopcroft, J. E., & Karp, R. M. (1973). An $n^{5/2}$ algorithm for maximum matchings in bipartite graphs. *SIAM Journal on Computing*, 2(4), 225-231.
pub fn bipartite_matching<U, V>(
    left_vertices: &[U],
    right_vertices: &[V],
    edges: &[(U, V)],
) -> Vec<(U, V)>
where
    U: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    let mut u_to_idx: HashMap<U, usize> = HashMap::new();
    let mut idx_to_u: Vec<U> = Vec::new();
    for u in left_vertices {
        if !u_to_idx.contains_key(u) {
            u_to_idx.insert(u.clone(), idx_to_u.len());
            idx_to_u.push(u.clone());
        }
    }

    let mut v_to_idx: HashMap<V, usize> = HashMap::new();
    let mut idx_to_v: Vec<V> = Vec::new();
    for v in right_vertices {
        if !v_to_idx.contains_key(v) {
            v_to_idx.insert(v.clone(), idx_to_v.len());
            idx_to_v.push(v.clone());
        }
    }

    let n_left = idx_to_u.len();
    let n_right = idx_to_v.len();

    let mut adj = vec![Vec::new(); n_left];
    for (u, v) in edges {
        if let (Some(&u_idx), Some(&v_idx)) = (u_to_idx.get(u), v_to_idx.get(v)) {
            adj[u_idx].push(v_idx);
        }
    }

    let mut pair_u = vec![usize::MAX; n_left];
    let mut pair_v = vec![usize::MAX; n_right];
    let mut dist = vec![0; n_left + 1];

    while hk_bfs(n_left, &adj, &pair_u, &pair_v, &mut dist) {
        for u in 0..n_left {
            if pair_u[u] == usize::MAX {
                hk_dfs(u, &adj, &mut pair_u, &mut pair_v, &mut dist);
            }
        }
    }

    let mut result = Vec::new();
    for u in 0..n_left {
        if pair_u[u] != usize::MAX {
            result.push((idx_to_u[u].clone(), idx_to_v[pair_u[u]].clone()));
        }
    }

    result
}

fn hk_bfs(
    n_left: usize,
    adj: &[Vec<usize>],
    pair_u: &[usize],
    pair_v: &[usize],
    dist: &mut [usize],
) -> bool {
    let mut queue = VecDeque::new();
    for u in 0..n_left {
        if pair_u[u] == usize::MAX {
            dist[u] = 0;
            queue.push_back(u);
        } else {
            dist[u] = usize::MAX;
        }
    }
    dist[n_left] = usize::MAX;

    while let Some(u) = queue.pop_front() {
        if dist[u] < dist[n_left] {
            for &v in &adj[u] {
                let next_u = pair_v[v];
                let d_target = if next_u == usize::MAX { n_left } else { next_u };
                if dist[d_target] == usize::MAX {
                    dist[d_target] = dist[u] + 1;
                    if d_target != n_left {
                        queue.push_back(d_target);
                    }
                }
            }
        }
    }

    dist[n_left] != usize::MAX
}

fn hk_dfs(
    u: usize,
    adj: &[Vec<usize>],
    pair_u: &mut [usize],
    pair_v: &mut [usize],
    dist: &mut [usize],
) -> bool {
    let n_left = pair_u.len();
    if u != n_left {
        for &v in &adj[u] {
            let next_u = pair_v[v];
            let d_target = if next_u == usize::MAX { n_left } else { next_u };

            if dist[d_target] == dist[u] + 1
                && (d_target == n_left || hk_dfs(d_target, adj, pair_u, pair_v, dist))
            {
                pair_v[v] = u;
                pair_u[u] = v;
                return true;
            }
        }
        dist[u] = usize::MAX;
        return false;
    }
    true
}
