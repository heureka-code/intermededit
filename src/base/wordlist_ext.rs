use crate::base::{HasWord, Letters};

/// Describes the typical interface of a wordlist that organises
/// the words in nested buckets.
///
/// The first level of buckets categorises the words by their length.
/// In every length's bucket there are buckets for the different
/// [Letters] bitmasks.
pub trait QueryableWordbucketList<'a> {
    /// The stored word type. As it only needs to implement [HasWord] it
    /// can also be e. g. a [super::TaggedWord] or some other wrapper.
    type W: HasWord;

    /// All words inside of the bucket determined by the length and [Letters] mask.
    /// If the bucket doesn't exist the iterator will be empty.
    fn words_of_bucket(&'a self, len: usize, letters: Letters)
    -> impl Iterator<Item = &'a Self::W>;
    /// Iterate all words stored in any bucket.
    fn iter_all(&'a self) -> impl Iterator<Item = &'a Self::W>;

    /// Get one word from the data structure if it exists.
    ///
    /// The default implementation calls [Iterator::next] on [Self::iter_all]
    /// and as it's deterministic this word will stay the same if the
    /// Data structure doesn't change but there are no guarantees about the returned word.
    fn get_any_word(&'a self) -> Option<&'a Self::W> {
        self.iter_all().next()
    }
    /// Get the number of words.
    ///
    /// The default implementation calls [Iterator::count] on [Self::iter_all]
    fn get_word_count(&'a self) -> usize {
        self.iter_all().count()
    }
}

pub trait InsertNewIntoWordbucketList<'a> {
    type W: HasWord;

    /// this method inserts the word and doesn't check if it is already in the list
    ///
    /// # Safety
    /// the caller has to ensure that the word is not already in the list
    unsafe fn insert_new_unchecked(&mut self, word: Self::W);
    /// this method inserts the word only if it is not already in the list
    ///
    /// - returns `false` if the word was already in the list
    /// - returns `true` if the word was newly inserted
    fn insert_new(&mut self, word: Self::W) -> bool;
}
