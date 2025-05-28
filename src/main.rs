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

fn main() {
    let start = Instant::now();
    let by_length = read_wordlist("wordlist-german.txt").expect("Wordlist file");
    println!(
        "Time taken for reading file and precomputing bitmasks of wordlist: {:?}",
        start.elapsed()
    );

    let tasks = vec![
        ("Herz", "rasen"),
        ("Bier", "Leber"),
        ("blau", "Alge"),
        ("Rhein", "raus"),
        ("Eis", "kalt"),
    ];
    let start = Instant::now();
    let _i: Vec<_> = tasks
        .par_iter()
        .map(|(start, target)| solution(&by_length, start, 10, target))
        .collect();
    for (start, _) in tasks {
        visual_benchmark_without_stopping(&by_length, Word::new(start), 10);
    }
    println!(
        "Time taken for completing the tasks (time for creating wordlist excluded): {:?}",
        start.elapsed()
    );
    print_len_histogram(&by_length);
}
