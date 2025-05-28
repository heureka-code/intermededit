use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use intermededit::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
            for reached in all_after_one_step(by_length, &rel_start) {
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

fn print_len_histogram(by_length: &AllWords) {
    for (ind, h) in by_length.iter().enumerate() {
        println!(
            "{ind:02} {} on {}",
            h.values().flatten().count(),
            h.values().count()
        );
    }
}

fn concurrent_edge_file_creation(all_words: &AllWords) {
    use edge_generation::*;
    let i = std::thread::spawn({
        let all_words = all_words.clone();
        move || {
            for len in 0..MAX_WORD_LEN {
                edges_for_insertion(&all_words, len).unwrap();
            }
        }
    });
    let d = std::thread::spawn({
        let all_words = all_words.clone();
        move || {
            for len in 1..MAX_WORD_LEN {
                edges_for_deletion(&all_words, len).unwrap();
            }
        }
    });
    let r = std::thread::spawn({
        let all_words = all_words.clone();
        move || {
            for len in 0..MAX_WORD_LEN {
                edges_for_substitution(&all_words, len).unwrap();
            }
        }
    });

    i.join().unwrap();
    d.join().unwrap();
    r.join().unwrap();
}

fn run_example_tasks_in_parallel(all_words: &AllWords, tasks: &[(&str, &str)]) {
    let start = Instant::now();
    let _i: Vec<_> = tasks
        .par_iter()
        .map(|(start, target)| solution(&all_words, start, 10, target))
        .collect();
    println!(
        "Time taken for completing the tasks (time for creating wordlist excluded): {:?}",
        start.elapsed()
    );
}

fn do_timed_way_generation_benchmark(all_words: &AllWords, tasks: &[(&str, &str)]) {
    println!("Starting visual benchmark of way generation:");
    let start = Instant::now();
    for (start, _) in tasks {
        visual_benchmark_without_stopping(&all_words, Word::new(start), 10);
    }
    println!("Time taken only for generating ways: {:?}", start.elapsed());
}

const TASKS: [(&str, &str); 5] = [
    ("Herz", "rasen"),
    ("Bier", "Leber"),
    ("blau", "Alge"),
    ("Rhein", "raus"),
    ("Eis", "kalt"),
];

fn main() {
    let start = Instant::now();
    let by_length = read_wordlist("wordlist-german.txt").expect("Wordlist file");
    println!(
        "Time taken for reading file and precomputing bitmasks of wordlist: {:?}",
        start.elapsed()
    );

    // concurrent_edge_file_creation(&by_length);

    run_example_tasks_in_parallel(&by_length, &TASKS);
    do_timed_way_generation_benchmark(&by_length, &TASKS);
    // print_len_histogram(&by_length);
}
