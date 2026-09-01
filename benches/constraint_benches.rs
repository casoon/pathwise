use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use pathwise::constraint::{BacktrackingOptions, Csp, backtracking};
use std::collections::HashMap;

fn create_n_queens_csp(n: usize) -> Csp<usize, usize> {
    let variables: Vec<usize> = (0..n).collect();
    let mut domains = HashMap::new();
    for v in &variables {
        domains.insert(*v, (0..n).collect());
    }

    let mut csp = Csp::new(variables.clone(), domains);

    for i in 0..n {
        for j in (i + 1)..n {
            csp.add_constraint(
                i,
                j,
                move |row1: &usize, col1: &usize, row2: &usize, col2: &usize| {
                    let r1: usize = *row1;
                    let r2: usize = *row2;
                    let c1: usize = *col1;
                    let c2: usize = *col2;
                    if c1 == c2 {
                        return false;
                    }
                    let row_diff = (r1.abs_diff(r2)) as isize;
                    let col_diff = (c1.abs_diff(c2)) as isize;
                    row_diff != col_diff
                },
            );
        }
    }

    csp
}

fn bench_constraint_solvers(c: &mut Criterion) {
    let mut group = c.benchmark_group("N-Queens Backtracking Comparison");

    for n in [4, 6, 8].iter() {
        let csp = create_n_queens_csp(*n);

        group.bench_with_input(BenchmarkId::new("Naive Backtracking", n), n, |b, _| {
            let options = BacktrackingOptions {
                use_mrv: false,
                use_lcv: false,
                use_ac3: false,
            };
            b.iter(|| backtracking(&csp, options))
        });

        group.bench_with_input(BenchmarkId::new("MRV + LCV + AC3", n), n, |b, _| {
            let options = BacktrackingOptions {
                use_mrv: true,
                use_lcv: true,
                use_ac3: true,
            };
            b.iter(|| backtracking(&csp, options))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_constraint_solvers);
criterion_main!(benches);
