use derive_more::Debug;
use std::collections::HashMap;

use crate::HasWord;
use crate::MAX_WORD_LEN;
use crate::base::Letters;
use crate::base::Word;
use crate::base::word::TWord;
use crate::base::wordlist_ext::InsertNewIntoWordbucketList;
use crate::base::wordlist_ext::QueryableWordbucketList;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LenLetWordBuckets<T> {
    buckets: Vec<HashMap<Letters, T>>,
}
impl<T> Default for LenLetWordBuckets<T> {
    fn default() -> Self {
        Self {
            buckets: (0..MAX_WORD_LEN + 2).map(|_| Default::default()).collect(),
        }
    }
}
#[allow(unused)]
/// Data structure for grouping data (typically words) by their length and [Letters] objects into
/// buckets for easy access
impl<T> LenLetWordBuckets<T> {
    pub fn get_mut(&mut self, len: usize, letters: &Letters) -> Option<&mut T> {
        self.buckets[len].get_mut(letters)
    }
    pub fn get(&self, len: usize, letters: &Letters) -> Option<&T> {
        self.buckets[len].get(letters)
    }
    pub fn get_or_default(&mut self, len: usize, letters: Letters) -> &mut T
    where
        T: Default,
    {
        self.buckets[len].entry(letters).or_default()
    }
    pub fn insert(&mut self, len: usize, letters: Letters, value: T) -> Option<T> {
        self.buckets[len].insert(letters, value)
    }
    pub fn remove(&mut self, len: usize, letters: &Letters) -> Option<T> {
        self.buckets[len].remove(letters)
    }

    /// Iterates over all values stored regardless in which bucket they are
    pub fn iter_all(&self) -> impl Iterator<Item = &T> {
        self.buckets.iter().flat_map(|hm| hm.values())
    }
    /// Iterate over all values in the bucket of a specified length with [Letters]
    pub fn iter_len(&self, len: usize) -> impl Iterator<Item = (&Letters, &T)> {
        self.buckets[len].iter()
    }
    /// Iterate over all values in the bucket of a specified length without [Letters]
    pub fn iter_len_values(&self, len: usize) -> impl Iterator<Item = &T> {
        self.buckets[len].values()
    }
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, T>> {
        self.buckets.iter()
    }

    /// If the stored generic type is iterable this will iterate over the items
    /// yielded by the instance stored for the given length and [Letters] instance.
    /// If no value is stored for the arguments the iterator will be empty.
    ///
    /// (stands for _inner_ iter)
    pub fn i_iter<'a, I>(&'a self, len: usize, letters: Letters) -> impl Iterator<Item = I>
    where
        &'a T: IntoIterator<Item = I>,
    {
        self.buckets[len]
            .get(&letters)
            .map(|t| t.into_iter())
            .into_iter()
            .flatten()
    }

    /// If the stored generic type is iterable this will iterate over all items
    /// yielded by any instance of `T` stored in any bucket.
    /// (This uses flattening on [Self::iter_all])
    ///
    /// (stands for _inner_ iter all)
    pub fn i_iter_all<'a, I>(&'a self) -> impl Iterator<Item = I>
    where
        &'a T: IntoIterator<Item = I>,
    {
        self.iter_all().flat_map(|t| t.into_iter())
    }

    /// If the stored generic type is iterable this will iterate over all items
    /// yielded by any instance of `T` stored in the bucket for the provided length.
    /// (This uses flattening on [Self::iter_len])
    ///
    /// (stands for _inner_ iter len)
    pub fn i_iter_len<'a, I>(&'a self, len: usize) -> impl Iterator<Item = (&'a Letters, I)>
    where
        &'a T: IntoIterator<Item = I>,
    {
        self.buckets[len]
            .iter()
            .flat_map(|(l, t)| t.into_iter().map(move |i| (l, i)))
    }

    pub fn get_for_len(&self, len: usize) -> &HashMap<Letters, T> {
        &self.buckets[len]
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct LenLetWordlist<'a>(LenLetWordBuckets<Vec<Word<'a>>>);

impl<'a> LenLetWordlist<'a> {
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, Vec<Word<'_>>>> {
        self.0.iter_lengths()
    }
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> QueryableWordbucketList<'a> for LenLetWordlist<'a> {
    type W = Word<'a>;

    fn words_of_bucket(&self, len: usize, letters: Letters) -> impl Iterator<Item = &Self::W> {
        self.0.i_iter(len, letters)
    }
    fn iter_all(&self) -> impl Iterator<Item = &Word<'_>> {
        self.0.i_iter_all()
    }
}
impl<'a> InsertNewIntoWordbucketList<'a> for LenLetWordlist<'a> {
    type W = Word<'a>;

    fn insert_new(&mut self, word: Self::W) -> bool {
        let v = self.0.get_or_default(word.len(), word.calc_letters());
        if v.contains(&word) {
            false
        } else {
            v.push(word);
            true
        }
    }
    unsafe fn insert_new_unchecked(&mut self, word: Self::W) {
        self.0
            .get_or_default(word.len(), word.calc_letters())
            .push(word);
    }
}

// Numbered

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TaggedLenLetWordlist<'a, N>(LenLetWordBuckets<Vec<TWord<'a, N>>>);

impl<'a, N> Default for TaggedLenLetWordlist<'a, N> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a, N> TaggedLenLetWordlist<'a, N> {
    pub fn iter_lengths(&self) -> impl Iterator<Item = &HashMap<Letters, Vec<TWord<'_, N>>>> {
        self.0.iter_lengths()
    }
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, N> QueryableWordbucketList<'a> for TaggedLenLetWordlist<'a, N> {
    type W = TWord<'a, N>;

    fn words_of_bucket(&self, len: usize, letters: Letters) -> impl Iterator<Item = &Self::W> {
        self.0.i_iter(len, letters)
    }
    fn iter_all(&self) -> impl Iterator<Item = &TWord<'_, N>> {
        self.0.i_iter_all()
    }
}
impl<'a, N: PartialEq> InsertNewIntoWordbucketList<'a> for TaggedLenLetWordlist<'a, N> {
    type W = TWord<'a, N>;

    fn insert_new(&mut self, word: Self::W) -> bool {
        let w = word.word();
        let v = self.0.get_or_default(w.len(), w.calc_letters());
        if v.contains(&word) {
            false
        } else {
            v.push(word);
            true
        }
    }
    unsafe fn insert_new_unchecked(&mut self, word: Self::W) {
        let w = word.word();
        self.0.get_or_default(w.len(), w.calc_letters()).push(word);
    }
}

impl<'a, N: PartialEq> TaggedLenLetWordlist<'a, N> {
    pub fn insert_new_with_ref(&mut self, word: TWord<'a, N>) -> &TWord<'a, N> {
        let w = word.word();
        let v: &mut Vec<_> = self.0.get_or_default(w.len(), w.calc_letters());
        if v.contains(&word) {
            for item in v {
                if item == &word {
                    return item;
                }
            }
            unreachable!()
        } else {
            v.push(word);
            v.last().unwrap()
        }
    }
}
