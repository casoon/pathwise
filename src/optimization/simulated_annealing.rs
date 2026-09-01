//! Simulated Annealing Optimization.

use crate::core::problem::OptimizationProblem;
use crate::core::solution::{SearchMetrics, Solution};

/// Options for Simulated Annealing schedule.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulatedAnnealingOptions {
    /// Initial temperature $T_0$.
    pub initial_temperature: f64,
    /// Cooling rate $\alpha \in (0, 1)$ per iteration step ($T_{k+1} = \alpha \cdot T_k$).
    pub cooling_rate: f64,
    /// Minimum temperature threshold to terminate annealing.
    pub min_temperature: f64,
    /// Maximum number of iteration steps.
    pub max_iterations: usize,
}

impl Default for SimulatedAnnealingOptions {
    fn default() -> Self {
        Self {
            initial_temperature: 1000.0,
            cooling_rate: 0.995,
            min_temperature: 0.001,
            max_iterations: 10_000,
        }
    }
}

/// Simple deterministic linear congruential PRNG for reproducible annealing choices.
struct LcgRng {
    state: u64,
}

impl LcgRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_f64(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let val = (self.state >> 11) as f64;
        val / (1u64 << 53) as f64
    }
}

/// Optimizes state solutions using Simulated Annealing.
///
/// Simulated Annealing avoids local optima by accepting worsening moves with probability
/// $P = \exp(\frac{\Delta E}{T})$ proportional to current temperature $T$.
///
/// # Complexity
///
/// - **Time Complexity**: $O(\text{iterations})$ bounded by cooling schedule.
/// - **Space Complexity**: $O(b)$ where $b$ is move neighborhood size.
///
/// # Requirements
///
/// - `P::Score` must be convertible or comparable to numeric value.
///
/// # Prefer this when
///
/// - Optimization landscape has many local optima or rugged terrain.
///
/// # References
///
/// - Kirkpatrick, S., Gelatt, C. D., & Vecchi, M. P. (1983). Optimization by simulated annealing. *Science*, 220(4598), 671-680.
pub fn simulated_annealing<P>(
    problem: &P,
    options: SimulatedAnnealingOptions,
    score_as_f64: impl Fn(P::Score) -> f64,
) -> Solution<P::State, P::Move, P::Score>
where
    P: OptimizationProblem,
{
    let mut current_state = problem.initial();
    let mut current_score = problem.score(&current_state);
    let mut best_state = current_state.clone();
    let mut best_score = current_score;

    let mut temp = options.initial_temperature;
    let mut rng = LcgRng::new(42);

    let mut nodes_expanded = 0;
    let mut nodes_visited = 1;

    for _ in 0..options.max_iterations {
        if temp < options.min_temperature {
            break;
        }

        nodes_expanded += 1;
        let moves: Vec<_> = problem.moves(&current_state).collect();
        if moves.is_empty() {
            break;
        }

        let idx = (rng.next_f64() * moves.len() as f64) as usize % moves.len();
        let mv = &moves[idx];
        let neighbor = problem.apply(&current_state, mv);
        nodes_visited += 1;
        let neighbor_score = problem.score(&neighbor);

        let delta = score_as_f64(neighbor_score) - score_as_f64(current_score);

        let accept = if delta > 0.0 {
            true
        } else {
            let prob = (delta / temp).exp();
            rng.next_f64() < prob
        };

        if accept {
            current_state = neighbor;
            current_score = neighbor_score;

            if current_score > best_score {
                best_state = current_state.clone();
                best_score = current_score;
            }
        }

        temp *= options.cooling_rate;
    }

    Solution::new(
        best_state,
        Vec::new(),
        best_score,
        SearchMetrics {
            nodes_expanded,
            nodes_visited,
        },
    )
}
