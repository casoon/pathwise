//! Maximum Flow (Edmonds-Karp Algorithm).

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Structure representing a flow network edge with capacity.
#[derive(Debug, Clone)]
pub struct FlowEdge<V, C> {
    /// Source vertex $u$.
    pub from: V,
    /// Destination vertex $v$.
    pub to: V,
    /// Edge capacity $c(u, v)$.
    pub capacity: C,
}

/// Computes the Maximum Flow from `source` to `sink` using the Edmonds-Karp algorithm.
///
/// Edmonds-Karp uses BFS to find shortest augmenting paths in residual capacity networks.
///
/// # Complexity
///
/// - **Time Complexity**: $O(V \cdot E^2)$ where $V$ is vertex count and $E$ is edge count.
/// - **Space Complexity**: $O(V + E)$ for residual graph representation.
///
/// # Returns
///
/// Returns the total maximum flow value from `source` to `sink`.
///
/// # References
///
/// - Edmonds, J., & Karp, R. M. (1972). Theoretical improvements in algorithmic efficiency for network flow problems. *Journal of the ACM*, 19(2), 248-264.
pub fn max_flow<V>(vertices: &[V], edges: &[FlowEdge<V, usize>], source: &V, sink: &V) -> usize
where
    V: Eq + Hash + Clone,
{
    if source == sink {
        return 0;
    }

    let mut v_to_idx: HashMap<V, usize> = HashMap::new();
    for v in vertices {
        if !v_to_idx.contains_key(v) {
            let idx = v_to_idx.len();
            v_to_idx.insert(v.clone(), idx);
        }
    }

    let src_idx = match v_to_idx.get(source) {
        Some(&idx) => idx,
        None => return 0,
    };
    let sink_idx = match v_to_idx.get(sink) {
        Some(&idx) => idx,
        None => return 0,
    };

    let n = v_to_idx.len();
    let mut capacity = vec![vec![0usize; n]; n];

    for edge in edges {
        if let (Some(&u), Some(&v)) = (v_to_idx.get(&edge.from), v_to_idx.get(&edge.to)) {
            capacity[u][v] += edge.capacity;
        }
    }

    let mut total_flow = 0;
    let mut parent = vec![usize::MAX; n];

    while bfs_augmenting_path(n, &capacity, src_idx, sink_idx, &mut parent) {
        let mut path_flow = usize::MAX;
        let mut v = sink_idx;

        while v != src_idx {
            let u = parent[v];
            path_flow = path_flow.min(capacity[u][v]);
            v = u;
        }

        v = sink_idx;
        while v != src_idx {
            let u = parent[v];
            capacity[u][v] -= path_flow;
            capacity[v][u] += path_flow;
            v = u;
        }

        total_flow += path_flow;
    }

    total_flow
}

fn bfs_augmenting_path(
    n: usize,
    capacity: &[Vec<usize>],
    src: usize,
    sink: usize,
    parent: &mut [usize],
) -> bool {
    parent.fill(usize::MAX);
    let mut visited = vec![false; n];

    let mut queue = VecDeque::new();
    queue.push_back(src);
    visited[src] = true;

    while let Some(u) = queue.pop_front() {
        if u == sink {
            return true;
        }

        for v in 0..n {
            if !visited[v] && capacity[u][v] > 0 {
                visited[v] = true;
                parent[v] = u;
                queue.push_back(v);
            }
        }
    }

    visited[sink]
}
