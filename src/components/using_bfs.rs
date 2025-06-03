use std::{collections::HashSet, sync::mpsc::Sender};

use crate::{AllWords, Word, all_after_one_step, base::*};

// panic!("More steps would be needed than possible (usize::MAX) steps were tried.")
#[derive(Debug, thiserror::Error, derive_more::Display)]
#[display("The step limit of {limit} is exceeded.")]
pub struct GraphIsTooBigForComponentAnalysis {
    reached: HashSet<Word>,
    limit: usize,
}

fn limited_component_classification(
    all_words: &AllWords,
    start: Word,
    max_distance: usize,
) -> Result<HashSet<Word>, GraphIsTooBigForComponentAnalysis> {
    let mut reached = HashSet::from_iter(vec![start.clone()]);
    let mut current = HashSet::from_iter(vec![&start]);

    for _step_no in 0..max_distance {
        let mut temp = HashSet::new();
        for rel_start in current {
            for w in all_after_one_step(all_words, rel_start) {
                if !reached.contains(w) {
                    temp.insert(w);
                    reached.insert(w.clone());
                }
            }
        }
        current = temp;
        if current.is_empty() {
            return Ok(reached);
        }
    }
    Err(GraphIsTooBigForComponentAnalysis {
        reached,
        limit: max_distance,
    })
}

pub struct BfsWordComponentClassification {
    max_distance: usize,
}

impl BfsWordComponentClassification {
    pub fn new_max() -> Self {
        Self {
            max_distance: usize::MAX,
        }
    }
    pub fn classify_words_into_components(
        &self,
        mut all_words: AllWords,
        tx_single: Sender<HashSet<Word>>,
    ) -> Result<(), GraphIsTooBigForComponentAnalysis> {
        while let Some(start) = all_words.get_any_word().cloned() {
            let reached = limited_component_classification(&all_words, start, self.max_distance)?;
            all_words.remove_iter_from_words(reached.iter());
            tx_single.send(reached).unwrap();
        }
        Ok(())
    }
}
