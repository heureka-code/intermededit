use std::{collections::HashMap, ops::Index};

use super::LenLetWordBuckets;
use crate::base::{
    HasWord, InsertWordbucketList, Letters, QueryableWordbucketList, TaggedWord, Word, WordlistExt,
};

#[derive(Debug, Default, Clone)]
pub struct NumberedLenLetWordlist {
    buckets: LenLetWordBuckets<Vec<TaggedWord<usize>>>,
    tag_map: HashMap<usize, Word>,
    index: usize,
}

impl InsertWordbucketList<Word> for NumberedLenLetWordlist {
    fn insert_new(&mut self, word: Word) {
        self.tag_map.insert(self.index, word.clone());
        self.buckets
            .get_or_default(self.index, word.calc_letters())
            .push(TaggedWord::new(self.index, word));
        self.index += 1;
    }
}

impl NumberedLenLetWordlist {
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, Vec<TaggedWord<usize>>>> {
        self.buckets.iter_lengths()
    }
    pub fn word_for_tag(&self, tag: &usize) -> Option<&Word> {
        self.tag_map.get(tag)
    }
}

impl WordlistExt for NumberedLenLetWordlist {
    type W = TaggedWord<usize>;
    fn remove_iter_from_words<'a>(
        &mut self,
        to_remove: impl Iterator<Item = &'a TaggedWord<usize>>,
    ) {
        use itertools::Itertools;
        for w in to_remove {
            let (len, letters) = (w.word().len(), w.word().calc_letters());
            if let Some(buc) = self.buckets.get_mut(len, &letters) {
                if let Some((pos, _)) = buc.iter().find_position(|a| a == &w) {
                    buc.remove(pos);
                }
            }
        }
    }
}

impl Index<usize> for NumberedLenLetWordlist {
    type Output = HashMap<Letters, Vec<TaggedWord<usize>>>;
    fn index(&self, index: usize) -> &Self::Output {
        self.buckets.get_for_len(index)
    }
}

impl QueryableWordbucketList for NumberedLenLetWordlist {
    type W = TaggedWord<usize>;

    fn words_of_bucket(&self, len: usize, letters: Letters) -> impl Iterator<Item = &Self::W> {
        self.buckets.i_iter(len, letters)
    }
    fn iter_all(&self) -> impl Iterator<Item = &TaggedWord<usize>> {
        self.buckets.i_iter_all()
    }
}
