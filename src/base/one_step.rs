use super::letters::LetterVariationsPerOperation;
use super::operations::{self, Delete, Insert, Replace};
use super::{AllWords, Word};

pub fn all_after_one_step<'a>(
    by_length: &'a AllWords,
    word: &'a Word,
) -> impl Iterator<Item = &'a Word> {
    find_after_operation::<Insert>(by_length, word)
        .chain(find_after_operation::<Delete>(by_length, word))
        .chain(find_after_operation::<Replace>(by_length, word))
}

pub fn find_after_operation<
    'a,
    Op: operations::Operation + FilterWordsForOperation + LetterVariationsPerOperation,
>(
    by_length: &'a AllWords,
    word: &'a Word,
) -> impl Iterator<Item = &'a Word> {
    let words_after_op = &by_length[(word.len() as i32 + Op::len_delta()) as usize];
    word.calc_letters()
        .operation_variations::<Op>()
        .flat_map(|letter_mask| words_after_op.get(&letter_mask))
        .flatten()
        .filter(move |other| <Op as FilterWordsForOperation>::filter_for_operation(word, other))
}

crate::base::operations::impl_operation_specific!(
    _filter_words_for_operation,
    pub trait FilterWordsForOperation: (operations::Operation) {
        (
            fn filter_for_operation(start: &Word, target: &Word) -> bool,
                insert: {
                    only_one_extra_letter(start, target)
                },
                delete: {
                    only_one_extra_letter(target, start)
                },
                replace: {
                    start
                        .chars()
                        .zip(target.chars())
                        .filter(|(a, b)| a != b)
                        .count()
                        < 2
                }
        )
    }
);

fn only_one_extra_letter(shorter: &Word, longer: &Word) -> bool {
    let mut offset = false;
    let mut longer_chars = longer.chars();
    for shortera in shorter.chars() {
        if Some(shortera) != longer_chars.next() {
            if !offset {
                offset = true;
                if Some(shortera) == longer_chars.next() {
                    continue;
                }
            }
            return false;
        }
    }
    true
}
