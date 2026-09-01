//! Topological Sort for Directed Acyclic Graphs (DAGs).

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Performs topological sorting on a Directed Acyclic Graph (DAG) using Kahn's algorithm.
///
/// Returns a valid linear ordering of vertices such that for every directed edge $u \to v$, $u$ comes before $v$.
///
/// # Complexity
///
/// - **Time Complexity**: $O(V + E)$ where $V$ is number of vertices and $E$ is number of directed edges.
/// - **Space Complexity**: $O(V)$ auxiliary space for in-degree counts and output queue.
///
/// # Returns
///
/// Returns `Some(Vec<V>)` containing topologically sorted vertices if the graph is acyclic, or `None` if a cycle is detected.
///
/// # References
///
/// - Kahn, A. B. (1962). Topological sorting of large networks. *Communications of the ACM*, 5(11), 558-562.
pub fn topological_sort<V>(vertices: &[V], edges: &[(V, V)]) -> Option<Vec<V>>
where
    V: Eq + Hash + Clone,
{
    let mut in_degrees: HashMap<V, usize> = HashMap::new();
    let mut adjacency: HashMap<V, Vec<V>> = HashMap::new();

    for v in vertices {
        in_degrees.entry(v.clone()).or_insert(0);
        adjacency.entry(v.clone()).or_default();
    }

    for (u, v) in edges {
        adjacency.entry(u.clone()).or_default().push(v.clone());
        *in_degrees.entry(v.clone()).or_insert(0) += 1;
    }

    let mut queue = VecDeque::new();
    for (v, deg) in &in_degrees {
        if *deg == 0 {
            queue.push_back(v.clone());
        }
    }

    let mut sorted = Vec::new();

    while let Some(u) = queue.pop_front() {
        sorted.push(u.clone());

        if let Some(neighbors) = adjacency.get(&u) {
            for v in neighbors {
                if let Some(deg) = in_degrees.get_mut(v) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(v.clone());
                    }
                }
            }
        }
    }

    if sorted.len() == vertices.len() {
        Some(sorted)
    } else {
        None // Cycle detected!
    }
}
