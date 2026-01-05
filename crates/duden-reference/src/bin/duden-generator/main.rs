use std::{collections::HashMap, io::Write};

use duden_reference::{
    HasWord, QueryableWordbucketList, TWord, Word,
    graphs::{Component, Components, WComponents, get_maybe_cached_graph},
    operations::InsertReplaceDelete,
};
use itertools::Itertools;
use petgraph::{
    csr::IndexType,
    graph::{DiGraph, NodeIndex},
    visit::Visitable,
};
use rand::seq::SliceRandom;

fn main() {
    let file_content = std::fs::read_to_string("lists/wordlist-orig-no-trailing-spaces.txt")
        .unwrap()
        .to_lowercase();

    let mut wordlist = duden_reference::TaggedLenLetWordlist::new();
    let mut word_lookup = HashMap::new();

    for line in file_content.lines() {
        if word_lookup.contains_key(line) {
            println!("Word already exists: {line}");
            continue;
        }
        let word = Word::new(line).unwrap().number_with(());
        let word = wordlist.insert_new_with_ref(word);
        word_lookup.insert(line, *word);
    }
    assert_eq!(word_lookup.len(), wordlist.iter_all().count());
    println!("wordlist complete");

    let graph: DiGraph<TWord<'_, ()>, (), u32> = get_maybe_cached_graph::<InsertReplaceDelete, _>(
        "data/ird/cached.graph",
        &wordlist,
        &word_lookup,
    );

    let components = WComponents::new(&graph);
    let biggest = components.biggest_to_smallest_comp().next().unwrap();

    let by_length = biggest
        .nodes()
        .map(|node| (graph[*node].word().len(), node))
        .into_group_map();

    let mut rng = rand::rng();

    println!("Start");
    let mut file = std::fs::File::create("s10tasks-big.txt").unwrap();
    for (_length, mut nodes) in by_length {
        println!("Start length={_length}");
        nodes.shuffle(&mut rng);
        let mut distances = (2..=10).flat_map(|i| vec![i as u32; 1000]).collect_vec();
        distances.shuffle(&mut rng);

        for (distance, node) in distances.iter().zip(nodes) {
            if let Some(target) = any_target_with_distance(&mut rng, &graph, node, distance) {
                // tasks.push((node, *distance, target));
                file.write_fmt(format_args!(
                    "{};{};{distance}\n",
                    graph[*node].word(),
                    graph[target].word()
                ))
                .unwrap();
            }
        }
    }
}

fn any_target_with_distance<N, Ix: IndexType, R: rand::Rng>(
    rng: &mut R,
    graph: &DiGraph<TWord<'_, N>, (), Ix>,
    from: &NodeIndex<Ix>,
    distance: &u32,
) -> Option<NodeIndex<Ix>> {
    let mut seen = graph.visit_map();
    let mut current = graph.visit_map();
    let mut new_map = graph.visit_map();
    seen.put(from.index());
    current.put(from.index());

    for _ in 0..*distance {
        for local_source in current.ones() {
            let local_source = NodeIndex::new(local_source);
            seen.put(local_source.index());
            for target in graph.neighbors_directed(local_source, petgraph::Direction::Outgoing) {
                if !seen.contains(target.index()) {
                    new_map.put(target.index());
                }
            }
        }

        std::mem::swap(&mut new_map, &mut current);
        new_map.clear();
    }
    current.remove(from.index());

    let reached = current.into_ones().collect_vec();

    if reached.is_empty() {
        return None;
    }

    let index = rng.random_range(0..reached.len());

    Some(NodeIndex::new(reached[index]))
}
