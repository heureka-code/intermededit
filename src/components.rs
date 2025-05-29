use std::{collections::HashSet, sync::mpsc::Sender};

use itertools::Itertools;

use crate::{AllWords, Word, all_after_one_step};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComponentAnalysis {
    DefinitlyComplete,
    TooBig,
}

pub fn limited_component_classification(
    all_words: &AllWords,
    start: Word,
    max_distance: usize,
) -> (HashSet<Word>, ComponentAnalysis) {
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
            return (reached, ComponentAnalysis::DefinitlyComplete);
        }
    }
    (reached, ComponentAnalysis::TooBig)
}

pub fn get_any_word(all_words: &AllWords) -> Option<&Word> {
    all_words.iter().flat_map(|b| b.values()).flatten().next()
}
pub fn get_word_count(all_words: &AllWords) -> usize {
    all_words.iter().flat_map(|b| b.values()).flatten().count()
}
fn remove_iter_from_words<'a>(all_words: &mut AllWords, to_remove: impl Iterator<Item = &'a Word>) {
    for w in to_remove {
        let (len, letters) = (w.len(), w.calc_letters());
        if let Some(buc) = all_words[len].get_mut(&letters) {
            if let Some((pos, _)) = buc.iter().find_position(|a| a == &w) {
                buc.remove(pos);
            }
        }
    }
}

pub fn classify_words_into_components(
    mut all_words: AllWords,
    max_distance: usize,
    tx_single: Sender<HashSet<Word>>,
    tx_unknown: Sender<HashSet<Word>>,
) {
    while let Some(start) = get_any_word(&all_words).cloned() {
        let (reached, analysis) = limited_component_classification(&all_words, start, max_distance);
        remove_iter_from_words(&mut all_words, reached.iter());
        match analysis {
            ComponentAnalysis::DefinitlyComplete => tx_single.send(reached).unwrap(),
            ComponentAnalysis::TooBig => tx_unknown.send(reached).unwrap(),
        }
    }
}
