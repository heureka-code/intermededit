pub mod model;
mod len_let_bucket;
pub mod one_step;

mod len_let_wordlist;
pub use len_let_bucket::LenLetWordBuckets;
pub use len_let_wordlist::LenLetWordlist;

pub use one_step::all_after_one_step;
pub use model::{Letters, Word, operations};

pub type AllWords = len_let_wordlist::LenLetWordlist;

pub trait WordlistExt {
    fn get_any_word(&self) -> Option<&Word>;
    fn get_word_count(&self) -> usize;
    fn remove_iter_from_words<'a>(&mut self, to_remove: impl Iterator<Item = &'a Word>);
}
