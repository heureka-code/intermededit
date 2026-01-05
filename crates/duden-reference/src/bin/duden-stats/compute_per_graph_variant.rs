use std::{collections::HashMap, sync::Arc};

use duden_reference::{
    FindAfterOperation, HasWord, TWord, TaggedLenLetWordlist,
    graphs::{Components, SComponents, get_maybe_cached_graph},
    operations::OperationShortage,
};
use petgraph::graph::DiGraph;

use crate::{
    comp_correlation_heatmaps::{
        all_word_lengths_per_component, avg_word_lengths_per_component,
        write_word_lengths_to_neighbors_count_to_file,
    },
    component_info::{write_component_size_amounts_to_file, write_singleton_words_to_file},
    diameter_center::diameter_bfs,
    msg_ctx::MSender,
};

pub fn compute<O: FindAfterOperation + OperationShortage>(
    messages: MSender,
    rayon_access: Arc<std::sync::Mutex<()>>,
    wordlist: &TaggedLenLetWordlist<'_, ()>,
    word_lookup: &HashMap<&str, TWord<'_, ()>>,
) {
    inner_compute::<O>(
        messages.scope(O::NAME),
        rayon_access,
        O::NAME,
        wordlist,
        word_lookup,
    )
}

fn inner_compute<O: FindAfterOperation>(
    messages: MSender,
    rayon_access: Arc<std::sync::Mutex<()>>,
    subfolder: &'static str,
    wordlist: &TaggedLenLetWordlist<'_, ()>,
    word_lookup: &HashMap<&str, TWord<'_, ()>>,
) {
    std::fs::create_dir_all(format!("data/{subfolder}")).unwrap();

    messages.send("start");

    let graph: DiGraph<_, (), u32> = get_maybe_cached_graph::<O, _>(
        &format!("data/{subfolder}/cached.graph"),
        wordlist,
        word_lookup,
    );

    messages.send(format!(
        "Got graph with {} nodes and {} edges",
        graph.node_count(),
        graph.edge_count()
    ));
    let components = SComponents::new(&graph);

    let _ = write_singleton_words_to_file(&components, subfolder, &graph);

    messages.send("write plot-data for component sizes to amounts");
    let _ = write_component_size_amounts_to_file(subfolder, &components);

    let word_length_per_comp: HashMap<u32, Vec<usize>> = components
        .groups()
        .map(|(id, words)| {
            (
                id,
                words.iter().map(|node| graph[*node].word().len()).collect(),
            )
        })
        .collect();

    let _ = write_word_lengths_to_neighbors_count_to_file(
        messages.scope("neighbors"),
        subfolder,
        &graph,
    );

    let _ = avg_word_lengths_per_component(
        messages.scope("avg-lengths"),
        subfolder,
        &word_length_per_comp,
    );

    let _ = all_word_lengths_per_component(
        messages.scope("all-lengths"),
        subfolder,
        &word_length_per_comp,
    );

    messages.send("finished normal stats");

    {
        messages.send("waiting for thread pool lock...");
        diameter_bfs(
            messages.scope("multi-bfs"),
            wordlist,
            rayon_access,
            &graph,
            &components,
            subfolder,
        )
        .unwrap();
    }

    messages.send("finished");
}
