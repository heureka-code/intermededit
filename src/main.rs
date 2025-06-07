mod utils;
use intermededit::{
    base::QueryableWordbucketList,
    components::ConnectedComponents,
    shortest_paths::{
        KeepBest, parallel_longest_shortest_path_targets, thread_collecting_best_from_components,
    },
    *,
};
use itertools::Itertools;
use utils::*;

#[allow(unused)]
const TASKS: [(&str, &str); 5] = [
    ("Herz", "rasen"),
    ("Bier", "Leber"),
    ("blau", "Alge"),
    ("Rhein", "raus"),
    ("Eis", "kalt"),
    // ("Zurufender", "Inschriften"),
];

fn main() {
    let all_words = time_function("Reading file and precomputing bitmasks of wordlist", || {
        expect_wordlist()
    });
    time_function("Completing given tasks", || {
        print_example_tasks_in_parallel(&all_words, &TASKS);
    });

    let all_edges = time_function("Computing all edges of graph", || {
        generate_edges_of_graph(&all_words).join().unwrap()
    });
    let components = ConnectedComponents::new(all_words.get_word_count(), &all_edges);

    let component_count = components.count();
    let biggest_size = components.biggest_component_size();

    let globally_best = time_function("Analyse components for diameter", || {
        let mut globally_best = KeepBest::new();
        for (words, edges) in components.take_words_edges() {
            let word_count = words.len();
            if word_count < 30 || edges.len() < 30 {
                continue;
            }
            let graph = edges_into_compressed_graph(edges.into_iter(), word_count);

            let (tx, rx) = std::sync::mpsc::channel();
            let thread = thread_collecting_best_from_components(word_count as u64, rx);
            parallel_longest_shortest_path_targets(&graph, tx);
            let best = thread.join().unwrap();

            let max: u8 = best.current_max();
            let best = best.into_iter().map(|(s, t)| {
                (
                    all_words.word_for_tag(graph[s] as usize),
                    all_words.word_for_tag(graph[t] as usize),
                )
            });
            globally_best.extend(max, best);
        }
        globally_best
    });

    println!("Component count: {component_count}, Biggest component: {biggest_size}",);

    println!(
        "Found diameter(s) of length {}: {}",
        globally_best.current_max(),
        globally_best
            .best()
            .map(|(s, t)| format!("{s} -> {t}"))
            .join(", ")
    );
    let gb = globally_best
        .take_vec()
        .into_iter()
        .map(|(s, t)| (s.clone(), t.clone()))
        .collect_vec();
    print_example_word_tasks_in_parallel(&all_words, &gb);
}
