//! More details about the model of words and the general concepts can be found in [base]
//!
//! The repository of this project lives here:
//! [https://github.com/heureka-code/intermededit](https://github.com/heureka-code/intermededit)

use std::collections::{HashMap, HashSet};

mod graph_generation;
pub mod shortest_paths;
use base::HasWord;
use base::QueryableWordbucketList;
pub use graph_generation::{edges_into_compressed_graph, generate_edges_of_graph};
pub mod base;
pub mod components;
mod read;
pub mod step_generation;
pub mod wordbucket_impls;
pub use base::Letters;
pub use base::Word;
pub use base::operations;
pub use wordbucket_impls::AllWords;

pub const MAX_WORD_LEN: usize = 50;
pub const DEFAULT_WORDLIST: &str = "wordlist-german.txt";
pub use read::{expect_wordlist, read_wordlist};
pub use step_generation::all_after_one_step;

pub fn find_way<L>(
    by_length: &L,
    start: Word,
    max_distance: usize,
    target: Word,
) -> Option<Vec<Word>>
where
    L: QueryableWordbucketList,
    L::W: PartialEq + std::hash::Hash,
{
    let mut reached_from = HashMap::<Word, Word>::new();
    let mut current = HashSet::from_iter(vec![&start]);

    for _step_no in 0..max_distance {
        let mut temp = HashSet::new();
        for rel_start in current {
            for reached in all_after_one_step(by_length, rel_start) {
                reached_from
                    .entry(reached.word().clone())
                    .or_insert(rel_start.clone());
                temp.insert(reached.word());
                if reached.word() == &target {
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
