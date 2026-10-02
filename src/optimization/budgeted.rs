//! Local search for problems whose evaluation is expensive, asynchronous and noisy.
//!
//! The other optimizers in this module assume a cheap, deterministic `score` and spend
//! evaluations freely. When every evaluation is a batch of network calls — tuning a
//! language-model program on a dataset, a simulation run, a benchmark — the evaluation
//! count is the budget, an evaluation must be awaitable, and "better" may need a test
//! rather than a comparison of two numbers.

use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;

use crate::core::problem::Problem;
use crate::core::rng::LcgRng;
use crate::core::solution::SearchMetrics;

/// An optimization problem with an expensive, possibly asynchronous and noisy evaluation.
///
/// Unlike [`OptimizationProblem`](crate::core::problem::OptimizationProblem), the score
/// does not have to be totally ordered: [`is_improvement`](Self::is_improvement) decides
/// whether a candidate beats the incumbent, so a noisy score can carry the per-sample
/// results a statistical test needs.
pub trait AsyncOptimizationProblem: Problem {
    /// Result of evaluating a state.
    type Score: Clone;

    /// Evaluates `state`. Called at most once per distinct state by
    /// [`budgeted_local_search`].
    fn evaluate(&self, state: &Self::State) -> impl Future<Output = Self::Score>;

    /// Whether `candidate` is better than `incumbent` by enough to move to it.
    fn is_improvement(&self, candidate: &Self::Score, incumbent: &Self::Score) -> bool;
}

/// Options for [`budgeted_local_search`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetedSearchOptions {
    /// Maximum number of evaluations, including the initial state.
    pub max_evaluations: usize,
    /// Seed for the order in which neighbors are tried.
    pub seed: u64,
}

impl Default for BudgetedSearchOptions {
    fn default() -> Self {
        Self {
            max_evaluations: 20,
            seed: 0,
        }
    }
}

/// One evaluation made during the search, in order.
#[derive(Debug, Clone, PartialEq)]
pub struct Evaluation<State, Score> {
    pub state: State,
    pub score: Score,
    /// Whether the search moved to this state.
    pub accepted: bool,
}

/// Outcome of [`budgeted_local_search`].
#[derive(Debug, Clone, PartialEq)]
pub struct BudgetedSolution<State, Score> {
    /// Best state found.
    pub state: State,
    pub score: Score,
    /// Every evaluation, in order; the first is the initial state.
    pub evaluations: Vec<Evaluation<State, Score>>,
    /// `true` when no neighbor of the final state improves on it, `false` when the budget
    /// ran out first.
    pub local_optimum: bool,
    pub metrics: SearchMetrics,
}

/// First-improvement local search under a budget of evaluations.
///
/// Starts at [`Problem::initial`], tries the neighbors of the current state in a seeded
/// random order, and moves to the first one that [`is_improvement`] accepts. Stops at a
/// local optimum or when the budget is spent. No state is evaluated twice.
///
/// First improvement rather than steepest ascent: with expensive evaluations, scoring
/// every neighbor before moving wastes most of the budget.
///
/// # Complexity
///
/// - **Evaluations**: at most `options.max_evaluations`, each awaited once.
/// - **Time**: $O(E \cdot b)$ besides evaluation, where $E$ is the number of evaluations and
///   $b$ the neighborhood size (generating and shuffling moves).
/// - **Space**: $O(E)$ for the evaluation log and the cache of evaluated states.
///
/// # Requirements
///
/// - `P::State: Eq + Hash` for the cache of evaluated states.
/// - The search is sequential: one evaluation at a time. Parallelism belongs inside
///   [`AsyncOptimizationProblem::evaluate`] (e.g. scoring a dataset concurrently).
/// - Runtime-agnostic: the returned future can be awaited on any executor.
///
/// # Prefer this when
///
/// - Each evaluation is costly enough that its count, not the search logic, is the budget.
/// - Scores are noisy and acceptance needs more than `>`.
///
/// # Consider instead
///
/// - [`hill_climbing`](crate::optimization::hill_climbing()) or
///   [`local_search`](crate::optimization::local_search()) when scoring is cheap.
///
/// # References
///
/// - Russell, S., & Norvig, P. (2020). *Artificial Intelligence: A Modern Approach* (4th ed.).
///   Pearson. Chapter 4.1.1 (first-choice hill climbing).
///
/// [`is_improvement`]: AsyncOptimizationProblem::is_improvement
pub async fn budgeted_local_search<P>(
    problem: &P,
    options: BudgetedSearchOptions,
) -> BudgetedSolution<P::State, P::Score>
where
    P: AsyncOptimizationProblem,
    P::State: Eq + Hash,
{
    let mut rng = LcgRng::new(options.seed);
    let mut evaluated: HashMap<P::State, P::Score> = HashMap::new();
    let mut evaluations = Vec::new();
    let mut metrics = SearchMetrics::default();

    let mut current = problem.initial();
    let mut current_score = problem.evaluate(&current).await;
    evaluated.insert(current.clone(), current_score.clone());
    evaluations.push(Evaluation {
        state: current.clone(),
        score: current_score.clone(),
        accepted: true,
    });
    metrics.nodes_visited = 1;

    let mut local_optimum = false;
    'search: while evaluations.len() < options.max_evaluations {
        metrics.nodes_expanded += 1;
        let mut moves: Vec<P::Move> = problem.moves(&current).collect();
        rng.shuffle(&mut moves);

        for mv in &moves {
            let neighbor = problem.apply(&current, mv);
            metrics.nodes_visited += 1;
            if evaluated.contains_key(&neighbor) {
                continue;
            }
            if evaluations.len() >= options.max_evaluations {
                break 'search;
            }
            let score = problem.evaluate(&neighbor).await;
            evaluated.insert(neighbor.clone(), score.clone());
            let accepted = problem.is_improvement(&score, &current_score);
            evaluations.push(Evaluation {
                state: neighbor.clone(),
                score: score.clone(),
                accepted,
            });
            if accepted {
                current = neighbor;
                current_score = score;
                continue 'search;
            }
        }
        local_optimum = true;
        break;
    }

    BudgetedSolution {
        state: current,
        score: current_score,
        evaluations,
        local_optimum,
        metrics,
    }
}
