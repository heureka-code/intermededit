use std::collections::HashMap;

use derive_more::Deref;

use super::LenLetWordBuckets;
use crate::base::{
    HasWord, InsertWordbucketList, Letters, QueryableWordbucketList, TaggedWord, Word,
};

#[derive(Debug, Deref, Clone, PartialEq)]
pub struct NumberedWord(TaggedWord<usize>);
impl HasWord for NumberedWord {
    fn word(&self) -> &Word {
        self.0.word()
    }
}

#[derive(Debug, Default, Clone)]
pub struct NumberedLenLetWordlist {
    buckets: LenLetWordBuckets<Vec<NumberedWord>>,
    tag_map: HashMap<usize, Word>,
    index: usize,
}
impl NumberedLenLetWordlist {
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, Vec<NumberedWord>>> {
        self.buckets.iter_lengths()
    }
    pub fn word_for_tag(&self, tag: &usize) -> Option<&Word> {
        self.tag_map.get(tag)
    }
    pub fn currently_used_index_count(&self) -> usize {
        self.index
    }
}

impl QueryableWordbucketList for NumberedLenLetWordlist {
    type W = NumberedWord;

    fn words_of_bucket(&self, len: usize, letters: Letters) -> impl Iterator<Item = &Self::W> {
        self.buckets.i_iter(len, letters)
    }
    fn iter_all(&self) -> impl Iterator<Item = &NumberedWord> {
        self.buckets.i_iter_all()
    }
}
impl InsertWordbucketList<Word> for NumberedLenLetWordlist {
    fn insert_new(&mut self, word: Word) {
        self.tag_map.insert(self.index, word.clone());
        self.buckets
            .get_or_default(self.index, word.calc_letters())
            .push(NumberedWord(TaggedWord::new(self.index, word)));
        self.index += 1;
    }
}
