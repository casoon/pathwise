use pathwise::core::problem::{Problem, SearchProblem};
use pathwise::search::{
    BeamSearchOptions, DfsOptions, astar, beam_search, best_first, bfs, dfs, iddfs, ucs,
};
use proptest::prelude::*;

/// Grid pathfinding problem on a 2D grid with obstacles.
#[derive(Clone)]
struct GridProblem {
    width: usize,
    height: usize,
    start: (usize, usize),
    goal: (usize, usize),
    obstacles: Vec<(usize, usize)>,
}

impl Problem for GridProblem {
    type State = (usize, usize);
    type Move = (isize, isize);

    fn initial(&self) -> Self::State {
        self.start
    }

    fn moves(&self, state: &Self::State) -> impl Iterator<Item = Self::Move> {
        let (x, y) = *state;
        let width = self.width;
        let height = self.height;

        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let obstacles = self.obstacles.clone();

        directions.into_iter().filter(move |&(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let pos = (nx as usize, ny as usize);
                !obstacles.contains(&pos)
            } else {
                false
            }
        })
    }

    fn apply(&self, state: &Self::State, mv: &Self::Move) -> Self::State {
        let (x, y) = *state;
        let (dx, dy) = *mv;
        ((x as isize + dx) as usize, (y as isize + dy) as usize)
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        *state == self.goal
    }
}

impl SearchProblem for GridProblem {
    type Cost = usize;

    fn step_cost(&self, _state: &Self::State, _mv: &Self::Move) -> Self::Cost {
        1
    }

    fn heuristic(&self, state: &Self::State) -> Self::Cost {
        // Manhattan distance (admissible and consistent for grid)
        let (x1, y1) = *state;
        let (x2, y2) = self.goal;
        (x1.abs_diff(x2)) + (y1.abs_diff(y2))
    }
}

#[test]
fn test_bfs_grid() {
    let problem = GridProblem {
        width: 5,
        height: 5,
        start: (0, 0),
        goal: (4, 4),
        obstacles: vec![(1, 1), (1, 2), (2, 1)],
    };

    let solution = bfs(&problem).expect("BFS should find path");
    assert_eq!(solution.state, (4, 4));
    assert_eq!(solution.cost, 8);
    assert_eq!(solution.moves.len(), 8);
}

#[test]
fn test_dfs_grid() {
    let problem = GridProblem {
        width: 5,
        height: 5,
        start: (0, 0),
        goal: (4, 4),
        obstacles: vec![(1, 1), (1, 2), (2, 1)],
    };

    let solution = dfs(&problem, DfsOptions::default()).expect("DFS should find path");
    assert_eq!(solution.state, (4, 4));
}

#[test]
fn test_iddfs_grid() {
    let problem = GridProblem {
        width: 5,
        height: 5,
        start: (0, 0),
        goal: (4, 4),
        obstacles: vec![(1, 1), (1, 2), (2, 1)],
    };

    let solution = iddfs(&problem, 10).expect("IDDFS should find path");
    assert_eq!(solution.state, (4, 4));
    assert_eq!(solution.cost, 8);
}

#[test]
fn test_ucs_grid() {
    let problem = GridProblem {
        width: 5,
        height: 5,
        start: (0, 0),
        goal: (4, 4),
        obstacles: vec![(1, 1), (1, 2), (2, 1)],
    };

    let solution = ucs(&problem).expect("UCS should find path");
    assert_eq!(solution.state, (4, 4));
    assert_eq!(solution.cost, 8);
}

#[test]
fn test_best_first_grid() {
    let problem = GridProblem {
        width: 5,
        height: 5,
        start: (0, 0),
        goal: (4, 4),
        obstacles: vec![(1, 1), (1, 2), (2, 1)],
    };

    let solution = best_first(&problem).expect("Best-first should find path");
    assert_eq!(solution.state, (4, 4));
}

#[test]
fn test_astar_grid() {
    let problem = GridProblem {
        width: 5,
        height: 5,
        start: (0, 0),
        goal: (4, 4),
        obstacles: vec![(1, 1), (1, 2), (2, 1)],
    };

    let solution = astar(&problem).expect("A* should find path");
    assert_eq!(solution.state, (4, 4));
    assert_eq!(solution.cost, 8);
    assert_eq!(solution.moves.len(), 8);
}

#[test]
fn test_beam_search_grid() {
    let problem = GridProblem {
        width: 5,
        height: 5,
        start: (0, 0),
        goal: (4, 4),
        obstacles: vec![(1, 1), (1, 2), (2, 1)],
    };

    let solution =
        beam_search(&problem, BeamSearchOptions::new(10)).expect("Beam search should find path");
    assert_eq!(solution.state, (4, 4));
}

#[test]
fn test_search_strategy_solve() {
    use pathwise::core::{AStarStrategy, BeamSearchStrategy, BfsStrategy, solve};

    let problem = GridProblem {
        width: 5,
        height: 5,
        start: (0, 0),
        goal: (4, 4),
        obstacles: vec![(1, 1), (1, 2), (2, 1)],
    };

    let sol_astar = solve(&problem, AStarStrategy).expect("AStarStrategy solve");
    let sol_bfs = solve(&problem, BfsStrategy).expect("BfsStrategy solve");
    let sol_beam = solve(&problem, BeamSearchStrategy::new(10)).expect("BeamSearchStrategy solve");

    assert_eq!(sol_astar.state, (4, 4));
    assert_eq!(sol_bfs.state, (4, 4));
    assert_eq!(sol_beam.state, (4, 4));
}

proptest! {
    #[test]
    fn prop_astar_matches_bfs_cost_on_unweighted_grids(
        width in 3..10usize,
        height in 3..10usize,
    ) {
        let problem = GridProblem {
            width,
            height,
            start: (0, 0),
            goal: (width - 1, height - 1),
            obstacles: vec![],
        };

        let bfs_sol = bfs(&problem).unwrap();
        let astar_sol = astar(&problem).unwrap();

        prop_assert_eq!(bfs_sol.cost, astar_sol.cost);
        prop_assert_eq!(bfs_sol.moves.len(), astar_sol.moves.len());
    }
}
