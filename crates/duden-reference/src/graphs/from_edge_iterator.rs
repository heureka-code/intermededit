use std::{collections::HashMap, hash::Hash};

use petgraph::Graph;

#[allow(unused)]
/// This takes an iterator of edges and returns a [petgraph::graph::UnGraph]
/// so it transforms the edges into an undirected and unweighted graph.
///
/// The yielded node values don't need to be continuous
/// a bijective function ([HashMap]) is used to assign every node
/// a unique index.
/// The input values are used as node weights.
pub fn edges_into_compressed_graph<N: Eq + Hash + Clone, EdgeKind>(
    edges: impl Iterator<Item = (N, N)>,
    node_count: usize,
) -> Graph<N, (), EdgeKind, u32>
where
    EdgeKind: petgraph::EdgeType,
{
    let mut graph = Graph::<N, (), EdgeKind, u32>::with_capacity(node_count, edges.size_hint().0);
    let mut transform = HashMap::new();

    for (s, t) in edges {
        let s = *transform
            .entry(s)
            .or_insert_with_key(|s| graph.add_node(s.clone()));
        let t = *transform
            .entry(t)
            .or_insert_with_key(|t| graph.add_node(t.clone()));
        graph.add_edge(s, t, ());
    }
    // doesn't hold here
    // debug_assert_eq!(graph.node_count(), node_count);
    graph
}

pub fn nodes_edges_into_compressed_graph<N: Eq + Hash + Clone, EdgeKind>(
    nodes: impl Iterator<Item = N>,
    edges: impl Iterator<Item = (N, N)>,
    node_count: usize,
) -> Graph<N, (), EdgeKind, u32>
where
    EdgeKind: petgraph::EdgeType,
{
    let mut graph = Graph::<N, (), EdgeKind, u32>::with_capacity(node_count, edges.size_hint().0);
    let mut transform = HashMap::new();

    for node in nodes {
        transform
            .entry(node)
            .or_insert_with_key(|node| graph.add_node(node.clone()));
    }

    for (s, t) in edges {
        let s = *transform
            .entry(s)
            .or_insert_with_key(|s| graph.add_node(s.clone()));
        let t = *transform
            .entry(t)
            .or_insert_with_key(|t| graph.add_node(t.clone()));
        graph.add_edge(s, t, ());
    }
    assert_eq!(graph.node_count(), node_count);
    graph
}
