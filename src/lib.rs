//! `pathwise` — generic search, optimization, and constraint-solving
//! building blocks for Rust.
//!
//! Concept and scope: see `README.md`. Implementation plan: `plan/`
//! (untracked, local only).
//!
//! Scaffold only — no logic yet. Module layout follows the layered
//! architecture from `plan/01-concept.md`:
//!
//! ```text
//! core
//!  ├── search
//!  ├── optimization
//!  └── constraint
//! graph
//! ```

// TODO(core): `Problem`/`SearchProblem` traits — State, Move, Cost, Score.
// mod core;

// TODO(search): bfs, dfs, iddfs, ucs, best_first, astar, beam_search.
// mod search;

// TODO(optimization): branch_and_bound, hill_climbing, local_search,
// simulated_annealing. (tabu_search, large_neighborhood_search: later)
// mod optimization;

// TODO(constraint): backtracking, forward_checking, propagation (AC-3),
// variable/value ordering (MRV, fail-first, least-constraining-value).
// mod constraint;

// TODO(graph): topological_sort. (bipartite_matching, max_flow,
// min_cost_flow, graph_coloring: later — evaluate petgraph integration
// first, see plan/01-concept.md)
// mod graph;
