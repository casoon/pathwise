use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use pathwise::core::problem::{Problem, SearchProblem};
use pathwise::search::{BeamSearchOptions, astar, beam_search, best_first, bfs, ucs};

#[derive(Clone)]
struct BenchmarkGridProblem {
    width: usize,
    height: usize,
    start: (usize, usize),
    goal: (usize, usize),
}

impl Problem for BenchmarkGridProblem {
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

        directions.into_iter().filter(move |&(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize
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

impl SearchProblem for BenchmarkGridProblem {
    type Cost = usize;

    fn step_cost(&self, _state: &Self::State, _mv: &Self::Move) -> Self::Cost {
        1
    }

    fn heuristic(&self, state: &Self::State) -> Self::Cost {
        let (x1, y1) = *state;
        let (x2, y2) = self.goal;
        (x1.abs_diff(x2)) + (y1.abs_diff(y2))
    }
}

fn bench_search_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grid Search Comparison");

    for size in [10, 25, 50].iter() {
        let problem = BenchmarkGridProblem {
            width: *size,
            height: *size,
            start: (0, 0),
            goal: (size - 1, size - 1),
        };

        group.bench_with_input(BenchmarkId::new("BFS", size), size, |b, _| {
            b.iter(|| bfs(&problem))
        });

        group.bench_with_input(BenchmarkId::new("UCS", size), size, |b, _| {
            b.iter(|| ucs(&problem))
        });

        group.bench_with_input(BenchmarkId::new("Best-First", size), size, |b, _| {
            b.iter(|| best_first(&problem))
        });

        group.bench_with_input(BenchmarkId::new("A*", size), size, |b, _| {
            b.iter(|| astar(&problem))
        });

        group.bench_with_input(
            BenchmarkId::new("Beam Search (W=10)", size),
            size,
            |b, _| b.iter(|| beam_search(&problem, BeamSearchOptions::new(10))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_search_algorithms);
criterion_main!(benches);
