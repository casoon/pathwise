use pathwise::solver::{AllDifferent, ConstraintSolver, Domain};

#[test]
fn test_domain_operations() {
    let mut domain = Domain::new(vec![1, 2, 3, 4]);
    assert_eq!(domain.len(), 4);
    assert!(domain.contains(&3));

    domain.remove(&3);
    assert_eq!(domain.len(), 3);
    assert!(!domain.contains(&3));

    domain.retain(|&v| v > 2);
    assert_eq!(domain.len(), 1);
    assert_eq!(domain.singleton(), Some(4));
}

#[test]
fn test_constraint_solver_all_different() {
    let mut solver = ConstraintSolver::new();
    let v1 = solver.add_variable(vec![1, 2]);
    let v2 = solver.add_variable(vec![1, 2]);

    solver.add_constraint(AllDifferent::new(vec![v1, v2]));

    let solution = solver
        .solve()
        .expect("AllDifferent CSP solution should exist");
    assert_ne!(solution.get(&v1), solution.get(&v2));
}
