mod letters;
mod word;
pub mod one_step;

pub use letters::Letters;
pub use word::Word;
pub use one_step::all_after_one_step;

pub type WordsOfLength = std::collections::HashMap<Letters, Vec<Word>>;
pub type AllWords = Vec<WordsOfLength>;

