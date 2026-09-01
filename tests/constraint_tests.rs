use pathwise::constraint::{BacktrackingOptions, Csp, ac3, backtracking};
use std::collections::HashMap;

#[test]
fn test_australia_map_coloring() {
    // Variables: WA, NT, SA, Q, NSW, V, T
    let variables = vec![
        "WA".to_string(),
        "NT".to_string(),
        "SA".to_string(),
        "Q".to_string(),
        "NSW".to_string(),
        "V".to_string(),
        "T".to_string(),
    ];

    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let mut domains = HashMap::new();
    for v in &variables {
        domains.insert(v.clone(), colors.clone());
    }

    let mut csp = Csp::new(variables, domains);

    let neighbors = vec![
        ("WA", "NT"),
        ("WA", "SA"),
        ("NT", "SA"),
        ("NT", "Q"),
        ("SA", "Q"),
        ("SA", "NSW"),
        ("SA", "V"),
        ("Q", "NSW"),
        ("NSW", "V"),
    ];

    for (v1, v2) in neighbors {
        csp.add_constraint(v1.to_string(), v2.to_string(), |_v1, val1, _v2, val2| {
            val1 != val2
        });
    }

    let options = BacktrackingOptions {
        use_mrv: true,
        use_lcv: true,
        use_ac3: true,
    };

    let solution = backtracking(&csp, options).expect("Map coloring solution should exist");

    assert_ne!(solution.get("WA"), solution.get("NT"));
    assert_ne!(solution.get("WA"), solution.get("SA"));
    assert_ne!(solution.get("NT"), solution.get("SA"));
    assert_ne!(solution.get("NT"), solution.get("Q"));
    assert_ne!(solution.get("SA"), solution.get("Q"));
    assert_ne!(solution.get("SA"), solution.get("NSW"));
    assert_ne!(solution.get("SA"), solution.get("V"));
    assert_ne!(solution.get("Q"), solution.get("NSW"));
    assert_ne!(solution.get("NSW"), solution.get("V"));
}

#[test]
fn test_ac3_pruning() {
    let variables = vec!["A".to_string(), "B".to_string()];
    let mut domains = HashMap::new();
    domains.insert("A".to_string(), vec![1, 2, 3]);
    domains.insert("B".to_string(), vec![3]);

    let mut csp = Csp::new(variables, domains);
    // Constraint: A < B
    csp.add_constraint(
        "A".to_string(),
        "B".to_string(),
        |_var1, val1, _var2, val2| val1 < val2,
    );

    let mut working_domains = csp.domains.clone();
    let is_consistent = ac3(&csp, &mut working_domains);

    assert!(is_consistent);
    assert_eq!(working_domains.get("A"), Some(&vec![1, 2]));
    assert_eq!(working_domains.get("B"), Some(&vec![3]));
}

#[test]
fn test_4_queens_csp() {
    let variables: Vec<usize> = (0..4).collect();
    let mut domains = HashMap::new();
    for v in &variables {
        domains.insert(*v, vec![0, 1, 2, 3]);
    }

    let mut csp = Csp::new(variables.clone(), domains);

    for i in 0..4 {
        for j in (i + 1)..4 {
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

    let options = BacktrackingOptions {
        use_mrv: true,
        use_lcv: false,
        use_ac3: false,
    };

    let solution = backtracking(&csp, options).expect("4-Queens solution should exist");
    assert_eq!(solution.len(), 4);
}
