//! Solution structures and search execution statistics.

/// Execution metrics captured during search or optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SearchMetrics {
    /// Total number of states expanded during search.
    pub nodes_expanded: usize,
    /// Total number of states visited/generated.
    pub nodes_visited: usize,
}

/// Represents a solution found by a search or optimization algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution<State, Move, Cost> {
    /// The final goal state or optimal state reached.
    pub state: State,
    /// The sequence of moves leading from the initial state to the final state.
    pub moves: Vec<Move>,
    /// Accumulated cost of the solution path.
    pub cost: Cost,
    /// Search metrics gathered during execution.
    pub metrics: SearchMetrics,
}

impl<State, Move, Cost> Solution<State, Move, Cost> {
    /// Creates a new `Solution` instance.
    pub fn new(state: State, moves: Vec<Move>, cost: Cost, metrics: SearchMetrics) -> Self {
        Self {
            state,
            moves,
            cost,
            metrics,
        }
    }
}
