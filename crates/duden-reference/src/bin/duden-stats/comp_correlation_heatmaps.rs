use std::{collections::HashMap, hash::Hash, io::Write};

use duden_reference::HasWord;
use itertools::Itertools;
use petgraph::graph::DiGraph;

use crate::msg_ctx::MSender;

pub fn avg_word_lengths_per_component(
    messages: MSender,
    subfolder: &'static str,
    word_length_per_comp: &HashMap<u32, Vec<usize>>,
) -> std::io::Result<()> {
    messages.send("compute average lengths for every component");
    let avg_word_lengths = word_length_per_comp
        .values()
        .map(|s| (s.iter().sum::<usize>() as f64 / s.len() as f64, s.len()))
        .collect_vec();

    messages.send("write plot-data for average word lengths per component size");
    let mut avg_file =
        std::fs::File::create(format!("data/{subfolder}/avg-word-lengths.data")).unwrap();
    for (avg, comp_size) in avg_word_lengths {
        avg_file.write_fmt(format_args!("{avg};{comp_size}\n"))?;
    }
    Ok(())
}

pub fn all_word_lengths_per_component(
    messages: MSender,
    subfolder: &'static str,
    word_length_per_comp: &HashMap<u32, Vec<usize>>,
) -> std::io::Result<()> {
    messages.send("tag all word lengths with component");
    let all_word_lengths = word_length_per_comp
        .values()
        .flat_map(|s| s.iter().map(|l| (l, s.len())))
        .collect_vec();
    messages.send("write plot-data for all word lengths per component size");
    let mut length_file =
        std::fs::File::create(format!("data/{subfolder}/word-lengths.data")).unwrap();
    for (wordl, comp_size) in all_word_lengths {
        length_file.write_fmt(format_args!("{wordl};{comp_size}\n"))?;
    }
    Ok(())
}

pub fn write_word_lengths_to_neighbors_count_to_file<W: HasWord + Eq + Hash>(
    messages: MSender,
    subfolder: &'static str,
    graph: &DiGraph<W, (), u32>,
) -> std::io::Result<()> {
    messages.send("write plot-data for word lengths to neighbors");
    let mut file =
        std::fs::File::create(format!("data/{subfolder}/word-length-neighbors.data")).unwrap();
    for source_idx in graph.node_indices() {
        let source_word = &graph[source_idx];
        let neighbor_count = graph
            .neighbors_undirected(source_idx)
            .map(|n| &graph[n])
            .unique()
            .filter(|x| *x != source_word)
            .count();
        file.write_fmt(format_args!(
            "{};{neighbor_count}\n",
            source_word.word().len()
        ))?;
    }
    Ok(())
}
