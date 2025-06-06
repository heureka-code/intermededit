use std::sync::mpsc::Sender;

use num::Unsigned;
use petgraph::graph::{NodeIndex, UnGraph};
use rayon::prelude::*;

use super::{HopDistanceBfs, KeepBest, VecNodeDistancesMap};

pub fn parallel_longest_shortest_path_targets<D: Unsigned + Copy + PartialOrd + Send + Sync>(
    graph: &UnGraph<u32, ()>,
    tx: Sender<(NodeIndex, D, KeepBest<NodeIndex, D>)>,
    word_count: usize,
) {
    graph.node_indices().par_bridge().for_each(|start| {
        let mut targets = KeepBest::new();

        let mut hd_bfs = HopDistanceBfs::new(
            &graph,
            start,
            VecNodeDistancesMap::<D>::with_max_size(word_count),
        );
        while let Some((nx, dst)) = hd_bfs.next(&graph) {
            targets.push(dst, nx);
        }
        let _ = tx.send((start, targets.current_max(), targets));
    });
}
