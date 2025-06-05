use super::{HasWord, Word};

/// A [Word] with some additional information, the _tag_.
/// This implements [HasWord] so this wrapper can be used mostly interchangably
/// where it is useful and appropriate.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TaggedWord<T> {
    tag: T,
    word: Word,
}

impl<T> TaggedWord<T> {
    /// Creates a new instance from a tag and a word
    pub fn new(tag: T, word: Word) -> Self {
        Self { tag, word }
    }
    /// Returns a reference to the inner tag.
    pub fn tag(&self) -> &T {
        &self.tag
    }
}

impl<T: Copy> TaggedWord<T> {
    /// If the tag type implements [Copy] this method exists and can be used to
    /// get an owned version of the stored tag.
    pub fn ctag(&self) -> T {
        self.tag
    }
}

/// Implementation to access the inner stored word without the tag.
impl<T> HasWord for TaggedWord<T> {
    fn word(&self) -> &Word {
        &self.word
    }
}
