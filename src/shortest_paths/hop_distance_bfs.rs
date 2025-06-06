use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use petgraph::{
    graph::NodeIndex,
    visit::{GraphRef, IntoNeighbors, VisitMap, Visitable},
};

pub struct VecNodeDistancesMap<D>(Vec<D>);
impl<D: num::Unsigned + Clone> VecNodeDistancesMap<D> {
    pub fn with_max_size(size: usize) -> Self {
        Self(vec![D::zero(); size])
    }
}
impl<D> Index<NodeIndex> for VecNodeDistancesMap<D> {
    type Output = D;
    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.0[index.index()]
    }
}
impl<D> IndexMut<NodeIndex> for VecNodeDistancesMap<D> {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self.0[index.index()]
    }
}

/// modified version of [petgraph::visit::Bfs] to include the hop distances
///
///
#[derive(Clone)]
pub struct HopDistanceBfs<N, VM, DM> {
    pub queue: VecDeque<N>,
    /// The map of discovered nodes
    pub discovered: VM,
    pub distances: DM,
}

impl<N, VM, DM> Default for HopDistanceBfs<N, VM, DM>
where
    VM: Default,
    DM: Default,
{
    fn default() -> Self {
        HopDistanceBfs {
            queue: VecDeque::new(),
            discovered: VM::default(),
            distances: DM::default(),
        }
    }
}

impl<N, VM, DM, D: num::Unsigned + Copy> HopDistanceBfs<N, VM, DM>
where
    N: Copy + PartialEq,
    DM: Index<N, Output = D> + IndexMut<N, Output = D>,
    VM: VisitMap<N>,
{
    pub fn new<G>(graph: G, start: N, distance_map: DM) -> Self
    where
        G: GraphRef + Visitable<NodeId = N, Map = VM>,
    {
        let mut discovered = graph.visit_map();
        discovered.visit(start);
        let mut queue = VecDeque::new();
        queue.push_front(start);
        HopDistanceBfs {
            queue,
            discovered,
            distances: distance_map,
        }
    }

    /// Return the next node in the bfs, or **None** if the traversal is done.
    pub fn next<G>(&mut self, graph: G) -> Option<(N, D)>
    where
        G: IntoNeighbors<NodeId = N>,
    {
        if let Some(node) = self.queue.pop_front() {
            for succ in graph.neighbors(node) {
                if self.discovered.visit(succ) {
                    self.distances[succ] = self.distances[node] + D::one();
                    self.queue.push_back(succ);
                }
            }
            return Some((node, self.distances[node]));
        }
        None
    }
}
