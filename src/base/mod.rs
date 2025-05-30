mod letters;
mod word;
pub mod one_step;

pub use letters::Letters;
pub use word::Word;
pub use one_step::all_after_one_step;

pub type WordsOfLength = std::collections::HashMap<Letters, Vec<Word>>;
pub type AllWords = Vec<WordsOfLength>;

pub fn get_any_word(all_words: &AllWords) -> Option<&Word> {
    all_words.iter().flat_map(|b| b.values()).flatten().next()
}
pub fn get_word_count(all_words: &AllWords) -> usize {
    all_words.iter().flat_map(|b| b.values()).flatten().count()
}
pub fn remove_iter_from_words<'a>(all_words: &mut AllWords, to_remove: impl Iterator<Item = &'a Word>) {
    use itertools::Itertools;
    for w in to_remove {
        let (len, letters) = (w.len(), w.calc_letters());
        if let Some(buc) = all_words[len].get_mut(&letters) {
            if let Some((pos, _)) = buc.iter().find_position(|a| a == &w) {
                buc.remove(pos);
            }
        }
    }
}


