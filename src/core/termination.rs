//! Search and optimization termination criteria.

/// Conditions under which search or optimization algorithms terminate execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationCondition<Score = usize> {
    /// Terminate after expanding at most `max_nodes` states.
    MaxExpandedNodes(usize),
    /// Terminate after visiting at most `max_visited` states.
    MaxVisitedNodes(usize),
    /// Terminate once objective score meets or exceeds `target_score`.
    ScoreTarget(Score),
}

impl<Score: Ord + Copy> TerminationCondition<Score> {
    /// Evaluates whether the current state meets the termination condition.
    pub fn should_terminate(
        &self,
        nodes_expanded: usize,
        nodes_visited: usize,
        current_score: Option<Score>,
    ) -> bool {
        match self {
            Self::MaxExpandedNodes(max) => nodes_expanded >= *max,
            Self::MaxVisitedNodes(max) => nodes_visited >= *max,
            Self::ScoreTarget(target) => match current_score {
                Some(score) => score >= *target,
                None => false,
            },
        }
    }
}
