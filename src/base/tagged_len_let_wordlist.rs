use std::{collections::HashMap, ops::Index};

use super::{HasWord, LenLetWordBuckets, Letters, Word, WordlistExt, model::TaggedWord};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TaggedLenLetWordlist<T>(LenLetWordBuckets<Vec<TaggedWord<T>>>);

impl<T> TaggedLenLetWordlist<T> {
    pub fn iter_all(&self) -> impl Iterator<Item = &TaggedWord<T>> {
        self.0.i_iter_all()
    }
    pub fn insert_new(&mut self, word: TaggedWord<T>) {
        self.0
            .get_or_default(word.word().len(), word.word().calc_letters())
            .push(word);
    }
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, Vec<TaggedWord<T>>>> {
        self.0.iter_lengths()
    }
}

impl<T: PartialEq> WordlistExt for TaggedLenLetWordlist<T>
where
    T: 'static,
{
    type W = TaggedWord<T>;
    fn get_any_word(&self) -> Option<&TaggedWord<T>> {
        self.iter_all().next()
    }
    fn get_word_count(&self) -> usize {
        self.iter_all().count()
    }
    fn remove_iter_from_words<'a>(&mut self, to_remove: impl Iterator<Item = &'a TaggedWord<T>>) {
        use itertools::Itertools;
        for w in to_remove {
            let (len, letters) = (w.word().len(), w.word().calc_letters());
            if let Some(buc) = self.0.get_mut(len, &letters) {
                if let Some((pos, _)) = buc.iter().find_position(|a| a == &w) {
                    buc.remove(pos);
                }
            }
        }
    }
}

impl<T> Index<usize> for TaggedLenLetWordlist<T> {
    type Output = HashMap<Letters, Vec<TaggedWord<T>>>;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.get_for_len(index)
    }
}
