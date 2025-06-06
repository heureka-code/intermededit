use petgraph::graph::UnGraph;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, mpsc::Receiver},
    thread::JoinHandle,
};

use crate::{
    all_after_one_step,
    base::{HasWord, QueryableWordbucketList},
    wordbucket_impls::NumberedLenLetWordlist,
};

fn graph_building_thread(
    capacity: usize,
    rx: Receiver<(u32, Arc<[u32]>)>,
) -> JoinHandle<Vec<(u32, u32)>> {
    std::thread::spawn(move || {
        let mut g = Vec::with_capacity(capacity);

        while let Ok((start, targets)) = rx.recv() {
            for target in targets.iter() {
                if start != *target {
                    g.push((start, *target));
                }
            }
        }
        g
    })
}

pub fn generate_edges_of_graph(
    numbered_words: &NumberedLenLetWordlist,
) -> JoinHandle<Vec<(u32, u32)>> {
    let wordcount = numbered_words.get_word_count();
    let (tx, rx) = std::sync::mpsc::channel::<(u32, Arc<[u32]>)>();
    let graph_gen_thread = graph_building_thread(wordcount, rx);

    numbered_words.iter_all().par_bridge().for_each(|nw| {
        tx.send((
            nw.ctag() as u32,
            all_after_one_step(numbered_words, nw.word())
                .map(|w| w.ctag() as u32)
                .collect(),
        ))
        .unwrap();
    });
    graph_gen_thread
}

/// This takes an iterator of edges and returns a [petgraph::graph::UnGraph]
/// so it transforms the edges into an undirected and unweighted graph.
///
/// The yielded node values don't need to be continuous
/// a bijective function ([HashMap]) is used to assign every node
/// a unique index.
/// The input values are used as node weights.
pub fn edges_into_compressed_graph<N: std::cmp::Eq + std::hash::Hash + Clone>(
    edges: impl Iterator<Item = (N, N)>,
    node_count: usize,
) -> UnGraph<N, (), u32> {
    let mut graph = UnGraph::<N, (), u32>::with_capacity(node_count, edges.size_hint().0);
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
    debug_assert_eq!(graph.node_count(), node_count);
    graph
}
