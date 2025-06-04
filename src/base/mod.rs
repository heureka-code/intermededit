mod has_word;
mod letters;
mod tagged_word;
mod word;
mod wordlist_kinds;

mod operations_macro;
#[macro_use]
pub mod operations;

pub use has_word::HasWord;
pub use letters::{LetterVariationsPerOperation, Letters};
pub use tagged_word::TaggedWord;
pub use word::Word;
pub use wordlist_kinds::{InsertWordbucketList, QueryableWordbucketList};

pub trait WordlistExt {
    type W: HasWord;
    fn remove_iter_from_words<'a>(&mut self, to_remove: impl Iterator<Item = &'a Self::W>)
    where
        Self::W: 'a;
}
