use std::{cmp::Reverse, collections::BinaryHeap, io::Write};

use duden_reference::{HasWord, graphs::Components};
use itertools::Itertools;
use petgraph::graph::{DiGraph, IndexType};

pub fn write_component_size_amounts_to_file<Ix: IndexType, C: Components<Ix>>(
    subfolder: &'static str,
    components: &C,
) -> std::io::Result<()> {
    let mut cc_file =
        std::fs::File::create(format!("data/{subfolder}/component-size-amounts.data")).unwrap();
    let comp2count = components
        .all_component_sizes()
        .sorted()
        .dedup_with_count()
        .map(|(amount, size)| (size, amount))
        .sorted();
    for (size, amount) in comp2count {
        cc_file.write_fmt(format_args!("{size};{amount}\n"))?;
    }
    Ok(())
}

pub fn singleton_words_shortest_to_longest<
    'a,
    Ix: IndexType,
    W: HasWord + Ord,
    C: Components<Ix>,
>(
    components: &C,
    graph: &'a DiGraph<W, (), Ix>,
) -> BinaryHeap<(Reverse<usize>, &'a W)> {
    components
        .all_nodes_with_comp_size()
        .flat_map(|(node, size)| (size == 1).then(|| &graph[node]))
        .map(|s| (std::cmp::Reverse(s.word().len()), s))
        .collect()
    /*
    println!("Top n");
    while let Some((l, w)) = singleton_words.pop() {
        println!("{}: {w}", l.0);
        std::thread::sleep(Duration::from_secs_f64(0.5));
    }*/
}

pub fn write_singleton_words_to_file<Ix: IndexType, W: HasWord + Ord, C: Components<Ix>>(
    components: &C,
    subfolder: &'static str,
    graph: &DiGraph<W, (), Ix>,
) -> std::io::Result<()> {
    let mut heap = singleton_words_shortest_to_longest(components, graph);

    let mut file = std::fs::File::create(format!(
        "data/{subfolder}/singleton-component-words-length-asc.data"
    ))
    .unwrap();
    while let Some((_, min)) = heap.pop() {
        file.write_fmt(format_args!("{}\n", min.word()))?;
    }
    Ok(())
}
