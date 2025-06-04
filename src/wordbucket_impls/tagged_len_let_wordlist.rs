use std::{collections::HashMap, hash::Hash, ops::Index};

use super::LenLetWordBuckets;
use crate::base::{HasWord, InsertNewIntoWordbucketList, Letters, QueryableWordbucketList, TaggedWord, Word, WordlistExt};

#[derive(Debug, Default, Clone)]
pub struct TaggedLenLetWordlist<T>(LenLetWordBuckets<Vec<TaggedWord<T>>>, HashMap<T, Word>);

impl<T: Copy + Hash + Eq> TaggedLenLetWordlist<T> {
    pub fn insert_new(&mut self, word: TaggedWord<T>) {
        self.1.insert(word.ctag(), word.word().clone());
        self.0
            .get_or_default(word.word().len(), word.word().calc_letters())
            .push(word);
    }
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, Vec<TaggedWord<T>>>> {
        self.0.iter_lengths()
    }
    pub fn word_for_tag(&self, tag: &T) -> Option<&Word> {
        self.1.get(tag)
    }
}

impl<T: PartialEq + Copy + Hash + Eq> WordlistExt for TaggedLenLetWordlist<T>
where
    T: 'static,
{
    type W = TaggedWord<T>;
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

impl<T> QueryableWordbucketList for TaggedLenLetWordlist<T> {
    type W = TaggedWord<T>;

    fn words_of_bucket(&self, len: usize, letters: Letters) -> impl Iterator<Item = &Self::W> {
        self.0.i_iter(len, letters)
    }
    fn iter_all(&self) -> impl Iterator<Item = &TaggedWord<T>> {
        self.0.i_iter_all()
    }
}
