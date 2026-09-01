#[cfg(feature = "petgraph")]
use pathwise::core::{AStarStrategy, BfsStrategy, UcsStrategy, solve};
#[cfg(feature = "petgraph")]
use pathwise::graph::PetgraphProblem;
#[cfg(feature = "petgraph")]
use petgraph::graph::DiGraph;

#[test]
#[cfg(feature = "petgraph")]
fn test_petgraph_adapter_astar_search() {
    let mut g = DiGraph::<&str, usize>::new();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");
    let d = g.add_node("D");

    g.add_edge(a, b, 2);
    g.add_edge(b, c, 3);
    g.add_edge(a, c, 10);
    g.add_edge(c, d, 1);

    let problem = PetgraphProblem::new(
        &g,
        a,
        d,
        |&w| w,
        |_| 0, // Admissible zero heuristic (Dijkstra mode)
    );

    let astar_sol = solve(&problem, AStarStrategy).expect("A* should find path in petgraph");
    assert_eq!(astar_sol.state, d);
    assert_eq!(astar_sol.cost, 6); // A -> B (2) -> C (3) -> D (1) = 6

    let ucs_sol = solve(&problem, UcsStrategy).expect("UCS should find path in petgraph");
    assert_eq!(ucs_sol.cost, 6);

    let bfs_sol = solve(&problem, BfsStrategy).expect("BFS should find path in petgraph");
    assert_eq!(bfs_sol.state, d);
    // BFS finds shortest move path (A -> C -> D = 2 steps)
    assert_eq!(bfs_sol.moves.len(), 2);
}
