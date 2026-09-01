//! Search algorithms module.
//!
//! Provides uninformed (BFS, DFS, IDDFS) and informed (UCS/Dijkstra, Greedy Best-First, A*, Beam Search) search strategies.

pub mod astar;
pub mod beam;
pub mod best_first;
pub mod bfs;
pub mod dfs;
pub mod iddfs;
pub mod ucs;

pub use astar::astar;
pub use beam::{BeamSearchOptions, beam_search};
pub use best_first::best_first;
pub use bfs::bfs;
pub use dfs::{DfsOptions, dfs};
pub use iddfs::iddfs;
pub use ucs::ucs;
