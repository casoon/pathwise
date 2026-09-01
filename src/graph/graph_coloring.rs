//! Graph Coloring (Welsh-Powell Algorithm).

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Computes a graph coloring mapping vertices to color indices using the Welsh-Powell heuristic.
///
/// Orders vertices by descending degree and greedily assigns the smallest valid color index to each vertex.
///
/// # Complexity
///
/// - **Time Complexity**: $O(V^2 + E)$ where $V$ is vertex count and $E$ is edge count.
/// - **Space Complexity**: $O(V + E)$ auxiliary graph memory.
///
/// # Returns
///
/// Returns `HashMap<V, usize>` mapping each vertex to its assigned color index ($0, 1, 2, \dots$).
///
/// # References
///
/// - Welsh, D. J., & Powell, M. B. (1967). An upper bound for the chromatic number of a graph and its application to timetabling problems. *The Computer Journal*, 10(1), 85-86.
pub fn graph_coloring<V>(vertices: &[V], edges: &[(V, V)]) -> HashMap<V, usize>
where
    V: Eq + Hash + Clone,
{
    let mut adj: HashMap<V, HashSet<V>> = HashMap::new();
    for v in vertices {
        adj.entry(v.clone()).or_default();
    }

    for (u, v) in edges {
        adj.entry(u.clone()).or_default().insert(v.clone());
        adj.entry(v.clone()).or_default().insert(u.clone());
    }

    let mut sorted_vertices: Vec<V> = vertices.to_vec();
    // Sort vertices by degree descending
    sorted_vertices.sort_by(|a, b| {
        let deg_a = adj.get(a).map_or(0, |neighbors| neighbors.len());
        let deg_b = adj.get(b).map_or(0, |neighbors| neighbors.len());
        deg_b.cmp(&deg_a)
    });

    let mut colors: HashMap<V, usize> = HashMap::new();

    for v in sorted_vertices {
        let mut used_colors = HashSet::new();

        if let Some(neighbors) = adj.get(&v) {
            for neighbor in neighbors {
                if let Some(&color) = colors.get(neighbor) {
                    used_colors.insert(color);
                }
            }
        }

        let mut color = 0;
        while used_colors.contains(&color) {
            color += 1;
        }

        colors.insert(v, color);
    }

    colors
}
