use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{
    Graph,
    algo::kosaraju_scc,
    graph::{DiGraph, IndexType, NodeIndex},
};

#[allow(unused)]
pub trait Component<'a, Ix: IndexType>: std::fmt::Debug {
    fn id(&self) -> u32;
    fn size(&self) -> usize {
        self.nodes().count()
    }
    fn nodes(&self) -> impl Iterator<Item = &'a NodeIndex<Ix>>;
}

pub trait Components<Ix: IndexType> {
    type Comp<'a>: Component<'a, Ix>
    where
        Self: 'a;
    fn component_id_for(&self, node: &NodeIndex<Ix>) -> u32;
    fn size_of_component(&self, component_id: u32) -> usize;
    fn groups(&self) -> impl Iterator<Item = (u32, &Vec<NodeIndex<Ix>>)>;
    fn size_of_component_of(&self, node: &NodeIndex<Ix>) -> usize {
        self.size_of_component(self.component_id_for(node))
    }
    fn all_component_sizes(&self) -> impl Iterator<Item = usize> {
        self.groups().map(|(_, v)| v.len())
    }
    fn all_nodes_with_comp_size(&self) -> impl Iterator<Item = (NodeIndex<Ix>, usize)> {
        self.groups().flat_map(|(id, v)| {
            let size = self.size_of_component(id);
            v.iter().map(move |n| (*n, size))
        })
    }
    fn nodes_of_component(&self, id: u32) -> impl Iterator<Item = &NodeIndex<Ix>> {
        self.groups()
            .flat_map(move |(i, nodes)| (i == id).then_some(nodes))
            .flatten()
    }
    fn biggest_to_smallest_comp_with_size<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::Comp<'a>, usize)>;
    fn biggest_to_smallest_comp<'a>(&'a self) -> impl Iterator<Item = Self::Comp<'a>> {
        self.biggest_to_smallest_comp_with_size().map(|(a, _)| a)
    }
}

/// strongly connected components of a directed graph
pub struct SComponents<Ix> {
    groups: Vec<Vec<NodeIndex<Ix>>>,
    node_to_component: HashMap<NodeIndex<Ix>, u32>,
}

impl<Ix: IndexType> SComponents<Ix> {
    pub fn new<V, E>(graph: &DiGraph<V, E, Ix>) -> Self {
        let groups = kosaraju_scc(graph);
        let node_to_component = groups
            .iter()
            .enumerate()
            .flat_map(|(id, nodes)| nodes.iter().map(move |n| (*n, id as u32)))
            .collect();
        Self {
            groups,
            node_to_component,
        }
    }
}

#[allow(unused)]
#[derive(derive_more::Debug)]
#[debug("SComp({id}, size={size})")]
pub struct SComp<'a, Ix> {
    id: u32,
    size: usize,
    components: &'a SComponents<Ix>,
}
impl<'a, Ix: IndexType> Component<'a, Ix> for SComp<'a, Ix> {
    fn id(&self) -> u32 {
        self.id
    }
    fn size(&self) -> usize {
        self.size
    }
    fn nodes(&self) -> impl Iterator<Item = &'a NodeIndex<Ix>> {
        self.components.nodes_of_component(self.id)
    }
}

impl<Ix: IndexType> Components<Ix> for SComponents<Ix> {
    type Comp<'a> = SComp<'a, Ix>;
    fn component_id_for(&self, node: &NodeIndex<Ix>) -> u32 {
        self.node_to_component[node]
    }
    fn size_of_component(&self, component_id: u32) -> usize {
        self.groups[component_id as usize].len()
    }

    fn groups(&self) -> impl Iterator<Item = (u32, &Vec<NodeIndex<Ix>>)> {
        self.groups
            .iter()
            .enumerate()
            .map(|(id, nodes)| (id as u32, nodes))
    }
    fn biggest_to_smallest_comp_with_size<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::Comp<'a>, usize)> {
        self.groups()
            .map(|(id, v)| {
                (
                    SComp {
                        id,
                        size: self.size_of_component(id),
                        components: self,
                    },
                    v.len(),
                )
            })
            .sorted_by(|(i1, l1), (i2, l2)| l2.cmp(l1).then(i2.id.cmp(&i1.id)))
    }
}

pub struct WComponents<Ix> {
    groups: Vec<Vec<NodeIndex<Ix>>>,
    node_to_component: HashMap<NodeIndex<Ix>, u32>,
}

impl<Ix: IndexType> WComponents<Ix> {
    pub fn new<V, E, D: petgraph::EdgeType>(graph: &Graph<V, E, D, Ix>) -> Self {
        let mut components = petgraph::unionfind::UnionFind::new(graph.node_count());
        for node in graph.node_indices() {
            for neighbor in graph.neighbors_directed(node, petgraph::Direction::Outgoing) {
                components.union(node.index() as u32, neighbor.index() as u32);
            }
        }

        let group_iter = graph
            .node_indices()
            .map(|node| (components.find(node.index() as u32), node))
            .into_group_map()
            .into_values();

        let mut groups = vec![];
        let mut node_to_component = HashMap::new();

        for (id, nodes) in group_iter.enumerate() {
            for n in nodes.iter() {
                node_to_component.insert(*n, id as u32);
            }
            groups.push(nodes);
        }

        Self {
            groups,
            node_to_component,
        }
    }
}
#[allow(unused)]
#[derive(derive_more::Debug)]
#[debug("WComp({id}, size={size})")]
pub struct WComp<'a, Ix> {
    id: u32,
    size: usize,
    components: &'a WComponents<Ix>,
}
impl<'a, Ix: IndexType> Component<'a, Ix> for WComp<'a, Ix> {
    fn id(&self) -> u32 {
        self.id
    }
    fn size(&self) -> usize {
        self.size
    }
    fn nodes(&self) -> impl Iterator<Item = &'a NodeIndex<Ix>> {
        self.components.nodes_of_component(self.id)
    }
}
impl<Ix: IndexType> Components<Ix> for WComponents<Ix> {
    type Comp<'a> = WComp<'a, Ix>;
    fn component_id_for(&self, node: &NodeIndex<Ix>) -> u32 {
        self.node_to_component[node]
    }
    fn size_of_component(&self, component_id: u32) -> usize {
        self.groups[component_id as usize].len()
    }

    fn groups(&self) -> impl Iterator<Item = (u32, &Vec<NodeIndex<Ix>>)> {
        self.groups
            .iter()
            .enumerate()
            .map(|(id, nodes)| (id as u32, nodes))
    }
    fn biggest_to_smallest_comp_with_size<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::Comp<'a>, usize)> {
        self.groups()
            .map(|(id, v)| {
                (
                    WComp {
                        id,
                        size: self.size_of_component(id),
                        components: self,
                    },
                    v.len(),
                )
            })
            .sorted_by(|(i1, l1), (i2, l2)| l2.cmp(l1).then(i2.id.cmp(&i1.id)))
    }
}
