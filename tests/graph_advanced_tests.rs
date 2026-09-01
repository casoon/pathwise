use pathwise::core::TerminationCondition;
use pathwise::graph::{FlowEdge, bipartite_matching, graph_coloring, max_flow};

#[test]
fn test_bipartite_matching() {
    let workers = vec!["W1", "W2", "W3", "W4"];
    let tasks = vec!["T1", "T2", "T3"];

    let edges = vec![
        ("W1", "T1"),
        ("W1", "T2"),
        ("W2", "T2"),
        ("W3", "T3"),
        ("W4", "T3"),
    ];

    let matching = bipartite_matching(&workers, &tasks, &edges);
    assert_eq!(matching.len(), 3); // Max possible matching size is min(|U|, |V|) = 3
}

#[test]
fn test_max_flow_edmonds_karp() {
    let vertices = vec!["S", "A", "B", "T"];
    let edges = vec![
        FlowEdge {
            from: "S",
            to: "A",
            capacity: 10,
        },
        FlowEdge {
            from: "S",
            to: "B",
            capacity: 10,
        },
        FlowEdge {
            from: "A",
            to: "B",
            capacity: 2,
        },
        FlowEdge {
            from: "A",
            to: "T",
            capacity: 8,
        },
        FlowEdge {
            from: "B",
            to: "T",
            capacity: 10,
        },
    ];

    let flow = max_flow(&vertices, &edges, &"S", &"T");
    assert_eq!(flow, 18);
}

#[test]
fn test_graph_coloring_welsh_powell() {
    let vertices = vec!["A", "B", "C", "D"];
    let edges = vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A"), ("A", "C")];

    let coloring = graph_coloring(&vertices, &edges);

    // Verify adjacent vertices do not share the same color
    for (u, v) in edges {
        assert_ne!(coloring.get(u), coloring.get(v));
    }
}

#[test]
fn test_termination_condition() {
    let term_nodes = TerminationCondition::<usize>::MaxExpandedNodes(100);
    assert!(term_nodes.should_terminate(100, 200, None));
    assert!(!term_nodes.should_terminate(99, 200, None));

    let term_score = TerminationCondition::<usize>::ScoreTarget(500);
    assert!(term_score.should_terminate(10, 10, Some(500)));
    assert!(!term_score.should_terminate(10, 10, Some(499)));
}
