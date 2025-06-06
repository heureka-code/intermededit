use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use intermededit::{
    base::QueryableWordbucketList,
    components::ConnectedComponents,
    shortest_paths::{parallel_longest_shortest_path_targets, thread_collecting_best_from_components},
    *,
};
use itertools::Itertools;
use rayon::prelude::*;

fn visual_benchmark_without_stopping(by_length: &AllWords, start: Word, max_distance: usize) {
    let mut reached_from = HashMap::<Word, Word>::new();
    let mut current = HashSet::from_iter(vec![&start]);
    let pb = indicatif::ProgressBar::new(max_distance as u64);
    pb.set_style(indicatif::ProgressStyle::with_template("{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message(format!("{start} ({max_distance})"));

    for _step_no in 0..max_distance {
        pb.inc(1);
        let mut temp = HashSet::new();
        for rel_start in current {
            for reached in all_after_one_step(by_length, rel_start) {
                reached_from
                    .entry(reached.clone())
                    .or_insert(rel_start.clone());
                temp.insert(reached);
            }
        }

        current = temp;
    }
    pb.finish();
}

fn solution(by_length: &AllWords, start: &str, max_distance: usize, target: &str) {
    let way = find_way(by_length, Word::new(start), max_distance, Word::new(target));
    if let Some(w) = way {
        println!("({} steps) {}", w.len() - 1, w.iter().join(" -> "));
    } else {
        println!("No way from {start} to {target} in {max_distance} steps with given words found!");
    }
}

#[allow(unused)]
fn print_len_histogram(by_length: &AllWords) {
    for (ind, h) in by_length.iter_lengths().enumerate() {
        println!(
            "{ind:02} {} on {}",
            h.values().flatten().count(),
            h.values().count()
        );
    }
}

fn run_example_tasks_in_parallel(all_words: &AllWords, tasks: &[(&str, &str)]) {
    let start = Instant::now();
    let _i: Vec<_> = tasks
        .par_iter()
        .map(|(start, target)| solution(all_words, start, 100, target))
        .collect();
    println!(
        "Time taken for completing the tasks (time for creating wordlist excluded): {:?}",
        start.elapsed()
    );
}

#[allow(unused)]
fn do_timed_way_generation_benchmark(all_words: &AllWords, tasks: &[(&str, &str)]) {
    println!("Starting visual benchmark of way generation:");
    let start = Instant::now();
    for (start, _) in tasks {
        visual_benchmark_without_stopping(all_words, Word::new(start), 10);
    }
    println!("Time taken only for generating ways: {:?}", start.elapsed());
}

const TASKS: [(&str, &str); 6] = [
    ("Herz", "rasen"),
    ("Bier", "Leber"),
    ("blau", "Alge"),
    ("Rhein", "raus"),
    ("Eis", "kalt"),
    ("Zurufender", "Inschriften"),
];

fn time_function<T>(label: &str, func: impl FnOnce() -> T) -> T {
    eprintln!("Start: {label}");
    let start = Instant::now();
    let value = func();
    eprintln!("Finished in {:?}: {label}", start.elapsed());
    value
}

fn main() {
    let all_words = time_function("Reading file and precomputing bitmasks of wordlist", || {
        expect_wordlist()
    });
    let all_edges = time_function("Computing all edges of graph", || {
        intermededit::generate_edges_of_graph(&all_words)
            .join()
            .unwrap()
    });
    let components = ConnectedComponents::new(all_words.get_word_count(), &all_edges);

    for (words, edges) in components.take_words_edges() {
        let word_count = words.len();
        if word_count < 30 || edges.len() < 30 {
            continue;
        }
        let graph = edges_into_compressed_graph(edges.into_iter(), word_count);

        let (tx, rx) = std::sync::mpsc::channel();
        let thread = thread_collecting_best_from_components(word_count as u64, rx);
        parallel_longest_shortest_path_targets(&graph, tx, word_count);
        let best = thread.join().unwrap();

        let max: u8 = best.current_max();
        let best = best
            .take_vec()
            .into_iter()
            .map(|(s, t)| {
                (
                    all_words.word_for_tag(graph[s] as usize).to_string(),
                    all_words.word_for_tag(graph[t] as usize).to_string(),
                )
            })
            .collect_vec();

        println!("{max} {best:?}");
    }
    // run_example_tasks_in_parallel(&by_length, &TASKS);
    //visual_classify_words_exhaustive(
    //    by_length,
    //    BufWriter::new(File::create_new("single-components-maxint.txt").unwrap()),
    //);
    // do_timed_way_generation_benchmark(&by_length, &TASKS);
    // print_len_histogram(&by_length);
}
