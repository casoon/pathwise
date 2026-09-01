//! Graph primitives module.
//!
//! Provides fundamental graph algorithms: Topological Sort, Maximum Bipartite Matching, Maximum Flow, Graph Coloring,
//! and optional `petgraph` integration.

pub mod bipartite_matching;
pub mod graph_coloring;
pub mod max_flow;
pub mod petgraph_integration;
pub mod topological_sort;

pub use bipartite_matching::bipartite_matching;
pub use graph_coloring::graph_coloring;
pub use max_flow::{FlowEdge, max_flow};
#[cfg(feature = "petgraph")]
pub use petgraph_integration::PetgraphProblem;
pub use topological_sort::topological_sort;
