use std::{
    collections::{HashMap, HashSet},
    thread::JoinHandle,
    time::Instant,
};

use intermededit::{
    base::{LetterVariationsPerOperation, QueryableWordbucketList},
    components::ConnectedComponents,
    operations::{Delete, Insert, Replace},
    shortest_paths::{KeepBest, parallel_longest_shortest_path_targets},
    step_generation::FilterWordsForOperation,
    *,
};
use itertools::Itertools;
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::GraphRef,
};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

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

#[allow(unused)]
fn concurrent_edge_file_creation(all_words: &AllWords) {
    use edge_generation::*;

    fn generate_thread<Op: FilterWordsForOperation + LetterVariationsPerOperation>(
        all_words: &AllWords,
    ) -> JoinHandle<()> {
        let all_words = all_words.clone();
        std::thread::spawn(move || {
            for len in 0..MAX_WORD_LEN {
                edges_for_operation::<Op>(&all_words, len).unwrap();
            }
        })
    }
    let i = generate_thread::<Insert>(all_words);
    let d = generate_thread::<Delete>(all_words);
    let r = generate_thread::<Replace>(all_words);

    i.join().unwrap();
    d.join().unwrap();
    r.join().unwrap();
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
        read_wordlist("wordlist-german.txt").expect("Wordlist file")
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

        let (tx, rx) = std::sync::mpsc::channel::<(NodeIndex, u8, KeepBest<NodeIndex, u8>)>();
        let thread = std::thread::spawn(move || {
            let pb = indicatif::ProgressBar::new(word_count as u64);
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
        });

        parallel_longest_shortest_path_targets(&graph, tx, word_count);

        let best = thread.join().unwrap();
        let max = best.current_max();
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
    /*

    let mut uf = union_find_classify_words_into_components(&by_length);
    // println!("finished");
    let comps = (0..uf.size())
        .map(|node| {
            (
                uf.find(node),
                by_length.word_for_tag(&node).unwrap().clone(),
            )
        })
        .into_group_map();

    let mut components_w = BufWriter::new(File::create_new("uf-components.txt").unwrap());
    let mut sizes = BufWriter::new(File::create_new("uf-component-sizes.txt").unwrap());
    for (_, d) in comps {
        let line = d.iter().sorted().join("\t");
        components_w.write_fmt(format_args!("{line}\n"));
        sizes.write_fmt(format_args!("{}\n", d.iter().count()));
    }*/

    /*
    intermededit::shortest_paths::find_shortest_paths_from_file(
        "uf-components.txt",
        BufWriter::new(File::create_new("uf-shortest-paths.txt").unwrap()),
    )
    .unwrap();
    */

    // run_example_tasks_in_parallel(&by_length, &TASKS);

    //visual_classify_words_exhaustive(
    //    by_length,
    //    BufWriter::new(File::create_new("single-components-maxint.txt").unwrap()),
    //);

    // concurrent_edge_file_creation(&by_length);

    // do_timed_way_generation_benchmark(&by_length, &TASKS);
    // print_len_histogram(&by_length);
}
