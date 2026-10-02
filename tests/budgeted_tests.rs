use std::cell::Cell;
use std::collections::HashSet;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use pathwise::Problem;
use pathwise::optimization::{
    AsyncOptimizationProblem, BudgetedSearchOptions, budgeted_local_search,
};

/// Minimal executor: the futures in these tests never wait.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
    }
}

/// Choose 3 of 8 items; an item's value is its index, the score is the sum.
/// Evaluations are counted to check the budget and the cache.
struct PickThree {
    calls: Cell<usize>,
    /// Required gain before a move counts as an improvement.
    margin: u32,
}

impl Problem for PickThree {
    type State = Vec<u32>;
    type Move = (usize, u32);

    fn initial(&self) -> Vec<u32> {
        vec![0, 1, 2]
    }

    fn moves(&self, state: &Vec<u32>) -> impl Iterator<Item = (usize, u32)> {
        let chosen: HashSet<u32> = state.iter().copied().collect();
        let unchosen: Vec<u32> = (0..8).filter(|i| !chosen.contains(i)).collect();
        (0..state.len())
            .flat_map(move |slot| unchosen.clone().into_iter().map(move |item| (slot, item)))
    }

    fn apply(&self, state: &Vec<u32>, &(slot, item): &(usize, u32)) -> Vec<u32> {
        let mut next = state.clone();
        next[slot] = item;
        next.sort();
        next
    }

    fn is_goal(&self, _: &Vec<u32>) -> bool {
        false
    }
}

impl AsyncOptimizationProblem for PickThree {
    type Score = u32;

    async fn evaluate(&self, state: &Vec<u32>) -> u32 {
        self.calls.set(self.calls.get() + 1);
        state.iter().sum()
    }

    fn is_improvement(&self, candidate: &u32, incumbent: &u32) -> bool {
        *candidate > incumbent + self.margin
    }
}

fn problem(margin: u32) -> PickThree {
    PickThree {
        calls: Cell::new(0),
        margin,
    }
}

#[test]
fn reaches_the_optimum_with_enough_budget() {
    let p = problem(0);
    let options = BudgetedSearchOptions {
        max_evaluations: 200,
        seed: 7,
    };
    let solution = block_on(budgeted_local_search(&p, options));
    assert_eq!(solution.state, vec![5, 6, 7]);
    assert_eq!(solution.score, 18);
    assert!(solution.local_optimum);
    assert_eq!(p.calls.get(), solution.evaluations.len());
    assert!(
        solution.evaluations[0].accepted,
        "the initial state is the first evaluation"
    );
}

#[test]
fn respects_the_budget() {
    let p = problem(0);
    let options = BudgetedSearchOptions {
        max_evaluations: 5,
        seed: 1,
    };
    let solution = block_on(budgeted_local_search(&p, options));
    assert_eq!(p.calls.get(), 5);
    assert_eq!(solution.evaluations.len(), 5);
    assert!(!solution.local_optimum);
    let best = solution
        .evaluations
        .iter()
        .filter(|e| e.accepted)
        .map(|e| e.score)
        .max()
        .unwrap();
    assert_eq!(
        solution.score, best,
        "the result is the last accepted state"
    );
}

#[test]
fn never_evaluates_a_state_twice() {
    let p = problem(0);
    let solution = block_on(budgeted_local_search(
        &p,
        BudgetedSearchOptions {
            max_evaluations: 200,
            seed: 3,
        },
    ));
    let distinct: HashSet<&Vec<u32>> = solution.evaluations.iter().map(|e| &e.state).collect();
    assert_eq!(distinct.len(), solution.evaluations.len());
}

#[test]
fn custom_acceptance_stops_early() {
    // Single swaps gain at most 7; a margin of 7 rejects every move.
    let p = problem(7);
    let solution = block_on(budgeted_local_search(
        &p,
        BudgetedSearchOptions {
            max_evaluations: 200,
            seed: 0,
        },
    ));
    assert_eq!(solution.state, vec![0, 1, 2]);
    assert!(solution.local_optimum);
    assert_eq!(
        solution.evaluations.iter().filter(|e| e.accepted).count(),
        1
    );
}

#[test]
fn same_seed_same_run() {
    let run = |seed| {
        let p = problem(0);
        block_on(budgeted_local_search(
            &p,
            BudgetedSearchOptions {
                max_evaluations: 12,
                seed,
            },
        ))
        .evaluations
        .into_iter()
        .map(|e| e.state)
        .collect::<Vec<_>>()
    };
    assert_eq!(run(42), run(42));
    assert_ne!(run(42), run(43));
}
