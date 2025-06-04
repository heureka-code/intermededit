use super::{HasWord, Word};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TaggedWord<T> {
    tag: T,
    word: Word,
}

impl<T> TaggedWord<T> {
    pub fn new(tag: T, word: Word) -> Self {
        Self { tag, word }
    }
    pub fn tag(&self) -> &T {
        &self.tag
    }
}

impl<T: Copy> TaggedWord<T> {
    pub fn ctag(&self) -> T {
        self.tag
    }
}

impl<T> HasWord for TaggedWord<T> {
    fn word(&self) -> &Word {
        &self.word
    }
}
