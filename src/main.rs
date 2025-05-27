use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use itertools::Itertools;
mod letters;
mod one_step;
mod word;
pub use letters::Letters;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
pub use word::Word;
pub type WordsOfLength = HashMap<Letters, Vec<Word>>;
pub type AllWords = Vec<WordsOfLength>;
const MAX_WORD_LEN: usize = 50;
pub use one_step::all_after_one_step;

fn read_wordlist(filename: &str) -> std::io::Result<AllWords> {
    use std::io::prelude::*;
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    let mut by_length = vec![WordsOfLength::new(); MAX_WORD_LEN + 2];

    for line in reader.lines() {
        let word = Word::new(&line?);
        assert!(word.len() < MAX_WORD_LEN);
        by_length[word.len()]
            .entry(word.calc_letters())
            .or_default()
            .push(word);
    }
    Ok(by_length)
}

fn find_way(
    by_length: &AllWords,
    start: Word,
    max_distance: usize,
    target: Word,
) -> Option<Vec<Word>> {
    let mut reached_from = HashMap::<Word, Word>::new();
    let mut current = HashSet::from_iter(vec![&start]);

    for _step_no in 0..max_distance {
        let mut temp = HashSet::new();
        for rel_start in current {
            for reached in all_after_one_step(by_length, &rel_start) {
                reached_from
                    .entry(reached.clone())
                    .or_insert(rel_start.clone());
                temp.insert(reached);
                if reached == &target {
                    let mut way = vec![target.clone()];
                    while way.last() != Some(&start) {
                        way.push(reached_from.get(way.last().unwrap()).unwrap().clone());
                    }
                    return Some(way.iter().cloned().rev().collect());
                }
            }
        }
        current = temp;
    }
    None
}

fn solution(by_length: &AllWords, start: &str, max_distance: usize, target: &str) {
    let way = find_way(by_length, Word::new(start), max_distance, Word::new(target));
    if let Some(w) = way {
        println!("({} steps) {}", w.len() - 1, w.iter().join(" -> "));
    } else {
        println!("No way from {start} to {target} in {max_distance} steps with given words found!");
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
    println!(
        "Time taken for completing the tasks (time for creating wordlist excluded): {:?}",
        start.elapsed()
    );
}
