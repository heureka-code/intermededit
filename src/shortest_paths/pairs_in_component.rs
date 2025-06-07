use std::{
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

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

pub fn thread_collecting_best_from_components<
    I: Unsigned + Copy + PartialOrd + Send + Sync + 'static,
>(
    word_count: u64,
    rx: Receiver<(NodeIndex, I, KeepBest<NodeIndex, I>)>,
) -> JoinHandle<KeepBest<(NodeIndex, NodeIndex), I>> {
    std::thread::spawn(move || {
        let pb = indicatif::ProgressBar::new(word_count);
        pb.set_style(indicatif::ProgressStyle::with_template(
                "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})"
            )
                .unwrap());

        let mut best = KeepBest::new();
        while let Ok((start, dst, targets)) = rx.recv() {
            pb.inc(1);
            best.extend(dst, targets.take_vec().into_iter().map(|t| (start, t)));
        }
        pb.finish();
        best
    })
}
