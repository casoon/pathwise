# pathwise

Generic search, optimization, and constraint-solving building blocks for
Rust — a small, curated set of algorithms behind one consistent trait-based
API, not an algorithms encyclopedia.

## Status

Concept phase. No implementation yet — see `plan/01-concept.md` (local,
untracked) for scope and design rationale.

## Scope

`pathwise` covers algorithms for exploring, searching, and optimizing over
a problem's state space:

- **search** — BFS, DFS, iterative deepening, uniform cost search,
  best-first search, A*, beam search
- **optimization** — branch and bound, hill climbing, local search,
  simulated annealing
- **constraint** — backtracking, forward checking, constraint propagation
  (AC-3), variable/value ordering heuristics
- **graph** — topological sort, with matching/flow/coloring evaluated
  later against integrating `petgraph` rather than reimplementing it

Deliberately out of scope: sorting and data structures that `std` or
established crates (e.g. `petgraph`) already cover well. `pathwise` only
implements an algorithm where the algorithm itself is the API, not a
drop-in replacement for `slice::sort()` or `BinaryHeap`.

## Design

All search/optimization strategies solve the same generic problem shape —
a `Problem` trait (state, moves, cost/score) that different strategies
(`astar`, `branch_and_bound`, `beam_search`, `simulated_annealing`, ...)
run against interchangeably. See `plan/01-concept.md` for the trait
sketch and the phased roadmap toward a constraint solver and scheduling
framework built on top of this crate.

## Installation

Not yet published to crates.io.

## License

MIT — see [LICENSE](LICENSE).
