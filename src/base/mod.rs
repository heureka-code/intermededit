mod len_let_bucket;
pub mod model;
pub mod one_step;

mod len_let_wordlist;

pub use len_let_bucket::LenLetWordBuckets;
pub use len_let_wordlist::LenLetWordlist;

mod tagged_len_let_wordlist;

pub use model::{HasWord, Letters, Word, operations};
pub use one_step::all_after_one_step;

pub type AllWords = len_let_wordlist::LenLetWordlist;

pub trait WordlistExt {
    type W: HasWord;
    fn get_any_word(&self) -> Option<&Self::W>;
    fn get_word_count(&self) -> usize;
    fn remove_iter_from_words<'a>(&mut self, to_remove: impl Iterator<Item = &'a Self::W>)
    where
        Self::W: 'a;
}

pub trait AnyBucketWordlist {
    type W: HasWord;
    fn words_of_bucket(&self, len: usize, letters: Letters) -> impl Iterator<Item = &Self::W>;
}
