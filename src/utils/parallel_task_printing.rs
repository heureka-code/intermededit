use intermededit::{AllWords, Word, base::QueryableWordbucketList, find_way};
use itertools::Itertools;
use rayon::prelude::*;

fn print_way_between_words<L>(by_length: &L, start: Word, max_distance: usize, target: Word)
where
    L: QueryableWordbucketList,
    L::W: std::hash::Hash + PartialEq,
{
    let way = find_way(by_length, start.clone(), max_distance, target.clone());
    if let Some(w) = way {
        println!("({} steps) {}", w.len() - 1, w.iter().join(" -> "));
    } else {
        println!("No way from {start} to {target} in {max_distance} steps with given words found!");
    }
}

pub fn print_example_tasks_in_parallel<L>(all_words: &L, tasks: &[(&str, &str)])
where
    L: QueryableWordbucketList + Sync,
    L::W: std::hash::Hash + PartialEq,
{
    let _i: Vec<_> = tasks
        .par_iter()
        .map(|(start, target)| {
            print_way_between_words(all_words, Word::new(start), usize::MAX, Word::new(target))
        })
        .collect();
}

pub fn print_example_word_tasks_in_parallel<L>(all_words: &L, tasks: &[(Word, Word)])
where
    L: QueryableWordbucketList + Sync,
    L::W: std::hash::Hash + PartialEq,
{
    let _i: Vec<_> = tasks
        .par_iter()
        .map(|(start, target)| {
            print_way_between_words(all_words, start.clone(), usize::MAX, target.clone())
        })
        .collect();
}
