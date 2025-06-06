use std::collections::HashMap;

use derive_more::Deref;

use super::LenLetWordBuckets;
use crate::base::{
    HasWord, InsertNewIntoWordbucketList, Letters, QueryableWordbucketList, TaggedWord, Word,
};

pub type WordNumber = usize;

#[derive(Debug, Deref, Clone, PartialEq)]
pub struct NumberedWord(TaggedWord<WordNumber>);
impl HasWord for NumberedWord {
    fn word(&self) -> &Word {
        self.0.word()
    }
}
impl NumberedWord {
    pub fn take_word(self) -> Word {
        self.0.take_word()
    }
}

#[derive(Debug, Default, Clone)]
pub struct NumberedLenLetWordlist {
    buckets: LenLetWordBuckets<Vec<NumberedWord>>,
    tag_map: Vec<Word>,
    index: WordNumber,
}
impl NumberedLenLetWordlist {
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, Vec<NumberedWord>>> {
        self.buckets.iter_lengths()
    }
    pub fn word_for_tag(&self, tag: WordNumber) -> &Word {
        &self.tag_map[tag as usize]
    }
    pub fn currently_used_index_count(&self) -> WordNumber {
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
impl InsertNewIntoWordbucketList<Word> for NumberedLenLetWordlist {
    fn insert_new(&mut self, word: Word) {
        assert_eq!(self.index as usize, self.tag_map.len());
        self.tag_map.push(word.clone());
        self.buckets
            .get_or_default(word.len(), word.calc_letters())
            .push(NumberedWord(TaggedWord::new(self.index, word)));
        self.index += 1;
    }
}
