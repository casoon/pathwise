use pathwise::core::problem::{OptimizationProblem, Problem};
use pathwise::optimization::{
    BranchAndBoundProblem, Coordination, HillClimbingOptions, LocalSearchOptions,
    SimulatedAnnealingOptions, branch_and_bound, hill_climbing, local_search, simulated_annealing,
};

/// 0-1 Knapsack Problem for Branch and Bound optimization testing.
struct KnapsackProblem {
    weights: Vec<usize>,
    values: Vec<usize>,
    capacity: usize,
}

impl Problem for KnapsackProblem {
    type State = (usize, usize, usize); // (index, current_weight, current_value)
    type Move = bool; // true = include item, false = skip item

    fn initial(&self) -> Self::State {
        (0, 0, 0)
    }

    fn moves(&self, state: &Self::State) -> impl Iterator<Item = Self::Move> {
        let (idx, curr_w, _) = *state;
        if idx >= self.weights.len() {
            vec![].into_iter()
        } else {
            let mut valid_moves = vec![false];
            if curr_w + self.weights[idx] <= self.capacity {
                valid_moves.push(true);
            }
            valid_moves.into_iter()
        }
    }

    fn apply(&self, state: &Self::State, mv: &Self::Move) -> Self::State {
        let (idx, curr_w, curr_v) = *state;
        if *mv {
            (
                idx + 1,
                curr_w + self.weights[idx],
                curr_v + self.values[idx],
            )
        } else {
            (idx + 1, curr_w, curr_v)
        }
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        state.0 == self.weights.len()
    }
}

impl BranchAndBoundProblem for KnapsackProblem {
    type Score = usize;

    fn score(&self, state: &Self::State) -> Self::Score {
        state.2
    }

    fn upper_bound(&self, state: &Self::State) -> Self::Score {
        let (idx, _, curr_v) = *state;
        let mut remaining_value = 0;
        for i in idx..self.values.len() {
            remaining_value += self.values[i];
        }
        curr_v + remaining_value
    }
}

/// 1D Integer function optimization problem ($f(x) = -(x - 5)^2 + 100$).
struct QuadraticOptimizationProblem;

impl Problem for QuadraticOptimizationProblem {
    type State = i32;
    type Move = i32;

    fn initial(&self) -> Self::State {
        0
    }

    fn moves(&self, state: &Self::State) -> impl Iterator<Item = Self::Move> {
        let x = *state;
        let mut moves = Vec::new();
        if x > -100 {
            moves.push(-1);
        }
        if x < 100 {
            moves.push(1);
        }
        moves.into_iter()
    }

    fn apply(&self, state: &Self::State, mv: &Self::Move) -> Self::State {
        state + mv
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        *state == 5
    }
}

impl OptimizationProblem for QuadraticOptimizationProblem {
    type Score = i32;

    fn score(&self, state: &Self::State) -> Self::Score {
        let x = *state;
        -(x - 5) * (x - 5) + 100
    }
}

#[test]
fn test_branch_and_bound_knapsack() {
    let problem = KnapsackProblem {
        weights: vec![2, 3, 4, 5],
        values: vec![3, 4, 5, 8],
        capacity: 5,
    };

    let solution = branch_and_bound(&problem, &Coordination::default())
        .expect("Branch & Bound should solve knapsack");
    assert_eq!(solution.cost, 8); // Item 3 (w=5, v=8)
}

#[test]
fn test_hill_climbing_quadratic() {
    let problem = QuadraticOptimizationProblem;
    let options = HillClimbingOptions {
        max_iterations: Some(100),
    };

    let solution = hill_climbing(&problem, options);
    assert_eq!(solution.state, 5);
    assert_eq!(solution.cost, 100);
}

#[test]
fn test_local_search_quadratic() {
    let problem = QuadraticOptimizationProblem;
    let options = LocalSearchOptions::default();

    let solution = local_search(&problem, options, &Coordination::default());
    assert_eq!(solution.state, 5);
    assert_eq!(solution.cost, 100);
}

#[test]
fn test_simulated_annealing_quadratic() {
    let problem = QuadraticOptimizationProblem;
    let options = SimulatedAnnealingOptions {
        initial_temperature: 100.0,
        cooling_rate: 0.95,
        min_temperature: 0.01,
        max_iterations: 1000,
    };

    let solution = simulated_annealing(&problem, options, |s| s as f64);
    assert_eq!(solution.state, 5);
    assert_eq!(solution.cost, 100);
}

#[test]
fn test_tabu_search_quadratic() {
    let problem = QuadraticOptimizationProblem;
    let options = pathwise::optimization::TabuSearchOptions {
        max_iterations: 100,
        tabu_tenure: 5,
    };

    let solution = pathwise::optimization::tabu_search(&problem, options);
    assert_eq!(solution.state, 5);
    assert_eq!(solution.cost, 100);
}

impl pathwise::optimization::LnsProblem for QuadraticOptimizationProblem {
    type PartialState = i32;

    fn destroy(&self, state: &Self::State, _intensity: f64) -> Self::PartialState {
        *state
    }

    fn repair(&self, partial: &Self::PartialState) -> Self::State {
        let val = *partial;
        if val < 5 {
            val + 1
        } else if val > 5 {
            val - 1
        } else {
            5
        }
    }
}

#[test]
fn test_lns_quadratic() {
    let problem = QuadraticOptimizationProblem;
    let options = pathwise::optimization::LnsOptions {
        max_iterations: 50,
        destroy_intensity: 0.2,
    };

    let solution = pathwise::optimization::large_neighborhood_search(
        &problem,
        options,
        &pathwise::optimization::Coordination::default(),
    );
    assert_eq!(solution.state, 5);
    assert_eq!(solution.cost, 100);
}
