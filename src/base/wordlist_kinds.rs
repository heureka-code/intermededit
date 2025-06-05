use super::{HasWord, Letters};

/// Describes the typical interface of a wordlist that organises
/// the words in nested buckets.
///
/// The first level of buckets categorises the words by their length.
/// In every length's bucket there are buckets for the different
/// [Letters] bitmasks.
///
/// The stored words only need to implement [super::TaggedWord] so they can
/// carry additional information.
pub trait QueryableWordbucketList {
    /// The stored word type. As it only needs to implement [HasWord] it
    /// can also be e. g. a [super::TaggedWord] or some other wrapper.
    type W: HasWord;

    /// All words inside of the bucket determined by the length and [Letters] mask.
    /// If the bucket doesn't exist the iterator will be empty.
    fn words_of_bucket(&self, len: usize, letters: Letters) -> impl Iterator<Item = &Self::W>;
    /// Iterate all words stored in any bucket.
    fn iter_all(&self) -> impl Iterator<Item = &Self::W>;

    /// Get one word from the data structure if it exists.
    ///
    /// The default implementation calls [Iterator::next] on [Self::iter_all]
    /// and as it's deterministic this word will stay the same if the
    /// Data structure doesn't change but their are no guarantees about the returned word.
    fn get_any_word(&self) -> Option<&Self::W> {
        self.iter_all().next()
    }
    /// Get the number of words.
    ///
    /// The default implementation calls [Iterator::count] on [Self::iter_all]
    fn get_word_count(&self) -> usize {
        self.iter_all().count()
    }
}

/// Marks a wordbucket list in which items can get inserted.
/// The insertable type is generic so that implementations can choose
/// if they want to accept wrapper types or something else.
///
/// Provides a method for inserting _new_ words into the data structure:
/// Inserts a new word into the data structure.
/// This method is not required to do any checks whether the value
/// is already contained. The caller is required to check this
/// if it is not clear from the use case.
pub trait InsertNewIntoWordbucketList<Insertable> {
    /// Inserts a new word into the data structure.
    /// This method is not required to do any checks whether the value
    /// is already contained. The caller is required to check this
    /// if it is not clear that this never happens from the use case.
    fn insert_new(&mut self, word: Insertable);
}
