use std::{
    collections::BinaryHeap,
    io::Write,
    sync::{Arc, Mutex},
};

use duden_reference::{HasWord, TWord, TaggedLenLetWordlist, graphs::Components};
use petgraph::{
    csr::IndexType,
    graph::{DiGraph, NodeIndex},
    visit::Visitable,
};
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::msg_ctx::MSender;

/// returns the nodes with the biggest distance to `from`
fn longest_path_from<N, Ix: IndexType>(
    graph: &DiGraph<TWord<'_, N>, (), Ix>,
    from: NodeIndex<Ix>,
) -> (usize, impl Iterator<Item = NodeIndex>) {
    let mut seen = graph.visit_map();
    let mut current = graph.visit_map();
    let mut new_map = graph.visit_map();
    seen.put(from.index());
    current.put(from.index());
    let mut distance = 0;

    loop {
        for local_source in current.ones() {
            let local_source: NodeIndex<Ix> = NodeIndex::new(local_source);
            seen.put(local_source.index());
            for target in graph.neighbors_directed(local_source, petgraph::Direction::Outgoing) {
                if !seen.contains(target.index()) {
                    new_map.put(target.index());
                }
            }
        }
        if new_map.is_clear() {
            current.remove(from.index());
            return (distance, current.into_ones().map(NodeIndex::new));
        }
        distance += 1;
        std::mem::swap(&mut new_map, &mut current);
        new_map.clear();
    }
}
pub fn diameter_bfs<'a, Ix: IndexType + Send + Sync, C: Components<Ix> + Sync>(
    messages: MSender,
    _wordlist: &TaggedLenLetWordlist<'a, ()>,
    rayon_access: Arc<Mutex<()>>,
    graph: &DiGraph<TWord<'_, ()>, (), Ix>,
    components: &C,
    subfolder: &'static str,
) -> std::io::Result<()> {
    let mut node_to_max_distance_heap: BinaryHeap<(u32, u32, NodeIndex<Ix>)> = {
        let _lock = rayon_access.lock().unwrap();
        messages.send("start with thread pool");
        let x = graph
            .node_indices()
            .par_bridge()
            .map(|from| {
                (
                    components.component_id_for(&from), // one component at a time
                    (longest_path_from(graph, from).0 as u32)
                        .min(components.size_of_component_of(&from) as u32 - 1), // ordered by distance
                    from,
                )
            })
            .collect();
        messages.send("computed complete max-heap, release thread pool");
        x
    };

    let open_file = |id| {
        let size = components.size_of_component(id);
        let _ = std::fs::create_dir_all(format!("data/{subfolder}/multi-bfs/{size}"));
        (
            id,
            std::fs::File::create(format!("data/{subfolder}/multi-bfs/{size}/{id}.data")).unwrap(),
        )
    };

    std::fs::remove_dir_all(format!("data/{subfolder}/multi-bfs")).unwrap();

    let mut singleton_file =
        std::fs::File::create(format!("data/{subfolder}/multi-bfs/1-singletons.data")).unwrap();

    let mut file = None;
    while let Some((comp_id, distance, from)) = node_to_max_distance_heap.pop() {
        if components.size_of_component(comp_id) == 1 {
            singleton_file.write_fmt(format_args!(
                "{comp_id};{distance};{}\n",
                graph[from].word()
            ))?;
            continue;
        }

        if file.as_ref().is_none_or(|(id, _)| *id != comp_id) {
            file = Some(open_file(comp_id));
        }
        file.as_mut()
            .unwrap()
            .1
            .write_fmt(format_args!("{distance};{}\n", graph[from].word()))?;
    }
    messages.send("finished writing");
    Ok(())
}
