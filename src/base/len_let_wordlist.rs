use std::{collections::HashMap, ops::Index};

use super::{LenLetWordBuckets, Letters, Word, WordlistExt};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct LenLetWordlist(LenLetWordBuckets<Vec<Word>>);

impl LenLetWordlist {
    pub fn iter_all(&self) -> impl Iterator<Item = &Word> {
        self.0.i_iter_all()
    }
    pub fn insert_new(&mut self, word: Word) {
        self.0
            .get_or_default(word.len(), word.calc_letters())
            .push(word);
    }
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, Vec<Word>>> {
        self.0.iter_lengths()
    }
}

impl WordlistExt for LenLetWordlist {
    fn get_any_word(&self) -> Option<&Word> {
        self.iter_all().next()
    }
    fn get_word_count(&self) -> usize {
        self.iter_all().count()
    }
    fn remove_iter_from_words<'a>(&mut self, to_remove: impl Iterator<Item = &'a Word>) {
        use itertools::Itertools;
        for w in to_remove {
            let (len, letters) = (w.len(), w.calc_letters());
            if let Some(buc) = self.0.get_mut(len, &letters) {
                if let Some((pos, _)) = buc.iter().find_position(|a| a == &w) {
                    buc.remove(pos);
                }
            }
        }
    }
}

impl Index<usize> for LenLetWordlist {
    type Output = HashMap<Letters, Vec<Word>>;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.get_for_len(index)
    }
}
