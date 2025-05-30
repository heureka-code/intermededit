use std::collections::{HashMap, HashSet};

mod base;
pub mod components;
pub mod edge_generation;
mod read;
pub mod shortest_paths;
pub use base::AllWords;
pub use base::Letters;
pub use base::Word;
pub use base::WordsOfLength;

pub const MAX_WORD_LEN: usize = 50;
pub use base::all_after_one_step;
pub use base::{get_any_word, get_word_count};
pub use read::read_wordlist;

pub fn find_way(
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
            for reached in all_after_one_step(by_length, rel_start) {
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

pub fn generate_and_traverse_all_ways_without_stopping(
    by_length: &AllWords,
    start: Word,
    max_distance: usize,
) {
    let mut reached_from = HashMap::<Word, Word>::new();
    let mut current = HashSet::from_iter(vec![&start]);

    for _step_no in 0..max_distance {
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
}
