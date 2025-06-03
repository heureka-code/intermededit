use super::model::letters::LetterVariationsPerOperation;
use super::operations::{self, Delete, Insert, Replace};
use super::{AnyBucketWordlist, HasWord, Word};

pub fn all_after_one_step<'a, L>(
    by_length: &'a L,
    word: &'a <L as AnyBucketWordlist>::W,
) -> impl Iterator<Item = &'a <L as AnyBucketWordlist>::W>
where
    L: AnyBucketWordlist,
{
    find_after_operation::<Insert, L>(by_length, word)
        .chain(find_after_operation::<Delete, L>(by_length, word))
        .chain(find_after_operation::<Replace, L>(by_length, word))
}

pub fn find_after_operation<
    'a,
    Op: operations::Operation + FilterWordsForOperation + LetterVariationsPerOperation,
    L,
>(
    by_length: &'a L,
    word: &'a <L as AnyBucketWordlist>::W,
) -> impl Iterator<Item = &'a <L as AnyBucketWordlist>::W>
where
    L: AnyBucketWordlist,
{
    let w: &Word = word.word();
    let target_length = (w.len() as i32 + Op::len_delta()) as usize;
    w.calc_letters()
        .operation_variations::<Op>()
        .flat_map(move |letter_mask| by_length.words_of_bucket(target_length, letter_mask))
        .filter(move |other| <Op as FilterWordsForOperation>::filter_for_operation(word, *other))
}

crate::base::operations::impl_operation_specific!(
    _filter_words_for_operation,
    pub trait FilterWordsForOperation: (operations::Operation) {
        (
            fn filter_for_operation{W: HasWord}(start: W, target: W) -> bool,
                insert: {
                    only_one_extra_letter(start, target)
                },
                delete: {
                    only_one_extra_letter(target, start)
                },
                replace: {
                    start.word()
                        .chars()
                        .zip(target.word().chars())
                        .filter(|(a, b)| a != b)
                        .count()
                        < 2
                }
        )
    }
);

fn only_one_extra_letter<W: HasWord>(shorter: W, longer: W) -> bool {
    let mut offset = false;
    let mut longer_chars = longer.word().chars();
    for shortera in shorter.word().chars() {
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
