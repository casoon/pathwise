use pathwise::graph::topological_sort;

#[test]
fn test_topological_sort_dag() {
    let vertices = vec!["shirt", "tie", "belt", "jacket", "pants", "shoes", "socks"];
    let edges = vec![
        ("shirt", "tie"),
        ("shirt", "belt"),
        ("tie", "jacket"),
        ("belt", "jacket"),
        ("pants", "belt"),
        ("pants", "shoes"),
        ("socks", "shoes"),
    ];

    let sorted = topological_sort(&vertices, &edges).expect("DAG should have topological sort");
    assert_eq!(sorted.len(), 7);

    // Verify ordering respects all directed edges
    let pos = |item: &&str| sorted.iter().position(|r| r == item).unwrap();
    for (u, v) in edges {
        assert!(pos(&u) < pos(&v), "{} should precede {}", u, v);
    }
}

#[test]
fn test_topological_sort_cycle_detection() {
    let vertices = vec!["A", "B", "C"];
    let edges = vec![("A", "B"), ("B", "C"), ("C", "A")];

    let sorted = topological_sort(&vertices, &edges);
    assert!(sorted.is_none(), "Cycle should return None");
}
