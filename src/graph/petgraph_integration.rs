//! `petgraph` integration adapter (available under `feature = "petgraph"`).

#[cfg(feature = "petgraph")]
use petgraph::Graph;
#[cfg(feature = "petgraph")]
use petgraph::graph::NodeIndex;
#[cfg(feature = "petgraph")]
use petgraph::visit::EdgeRef;

#[cfg(feature = "petgraph")]
use crate::core::problem::{Problem, SearchProblem};

/// Adapter converting a [`petgraph::Graph`] into a [`SearchProblem`].
#[cfg(feature = "petgraph")]
pub struct PetgraphProblem<'a, N, E, Ty, Ix, Cost, WeightFn, HeuristicFn>
where
    Ty: petgraph::EdgeType,
    Ix: petgraph::graph::IndexType,
{
    graph: &'a Graph<N, E, Ty, Ix>,
    start: NodeIndex<Ix>,
    goal: NodeIndex<Ix>,
    weight_fn: WeightFn,
    heuristic_fn: HeuristicFn,
    _phantom: std::marker::PhantomData<Cost>,
}

#[cfg(feature = "petgraph")]
impl<'a, N, E, Ty, Ix, Cost, WeightFn, HeuristicFn>
    PetgraphProblem<'a, N, E, Ty, Ix, Cost, WeightFn, HeuristicFn>
where
    Ty: petgraph::EdgeType,
    Ix: petgraph::graph::IndexType,
    WeightFn: Fn(&E) -> Cost,
    HeuristicFn: Fn(NodeIndex<Ix>) -> Cost,
{
    /// Creates a new `PetgraphProblem` adapter.
    pub fn new(
        graph: &'a Graph<N, E, Ty, Ix>,
        start: NodeIndex<Ix>,
        goal: NodeIndex<Ix>,
        weight_fn: WeightFn,
        heuristic_fn: HeuristicFn,
    ) -> Self {
        Self {
            graph,
            start,
            goal,
            weight_fn,
            heuristic_fn,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "petgraph")]
impl<'a, N, E, Ty, Ix, Cost, WeightFn, HeuristicFn> Problem
    for PetgraphProblem<'a, N, E, Ty, Ix, Cost, WeightFn, HeuristicFn>
where
    Ty: petgraph::EdgeType,
    Ix: petgraph::graph::IndexType,
    WeightFn: Fn(&E) -> Cost,
    HeuristicFn: Fn(NodeIndex<Ix>) -> Cost,
{
    type State = NodeIndex<Ix>;
    type Move = NodeIndex<Ix>;

    fn initial(&self) -> Self::State {
        self.start
    }

    fn moves(&self, state: &Self::State) -> impl Iterator<Item = Self::Move> {
        let targets: Vec<NodeIndex<Ix>> =
            self.graph.edges(*state).map(|edge| edge.target()).collect();
        targets.into_iter()
    }

    fn apply(&self, _state: &Self::State, mv: &Self::Move) -> Self::State {
        *mv
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        *state == self.goal
    }
}

#[cfg(feature = "petgraph")]
impl<'a, N, E, Ty, Ix, Cost, WeightFn, HeuristicFn> SearchProblem
    for PetgraphProblem<'a, N, E, Ty, Ix, Cost, WeightFn, HeuristicFn>
where
    Ty: petgraph::EdgeType,
    Ix: petgraph::graph::IndexType,
    Cost: Copy + Ord + std::ops::Add<Output = Cost> + Default,
    WeightFn: Fn(&E) -> Cost,
    HeuristicFn: Fn(NodeIndex<Ix>) -> Cost,
{
    type Cost = Cost;

    fn step_cost(&self, state: &Self::State, mv: &Self::Move) -> Self::Cost {
        let edge = self.graph.edges(*state).find(|e| e.target() == *mv);

        match edge {
            Some(e) => (self.weight_fn)(e.weight()),
            None => Self::Cost::default(),
        }
    }

    fn heuristic(&self, state: &Self::State) -> Self::Cost {
        (self.heuristic_fn)(*state)
    }
}
