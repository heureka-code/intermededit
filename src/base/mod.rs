mod len_let_bucket;
mod letters;
pub mod one_step;
#[macro_use]
pub mod operations;
#[macro_use]
mod operations_macro;
mod len_let_wordlist;
mod word;
pub use len_let_bucket::LenLetWordBuckets;
pub use len_let_wordlist::LenLetWordlist;

pub use letters::Letters;
pub use one_step::all_after_one_step;
pub use word::Word;

pub type AllWords = len_let_wordlist::LenLetWordlist;

pub trait WordlistExt {
    fn get_any_word(&self) -> Option<&Word>;
    fn get_word_count(&self) -> usize;
    fn remove_iter_from_words<'a>(&mut self, to_remove: impl Iterator<Item = &'a Word>);
}
