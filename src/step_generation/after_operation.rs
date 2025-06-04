use super::check_for_change::{only_one_different_letter, only_one_extra_letter};
use crate::{
    Word,
    base::{HasWord, LetterVariationsPerOperation, QueryableWordbucketList, operations::Operation},
};

pub fn find_after_operation<
    'a,
    Op: Operation + FilterWordsForOperation + LetterVariationsPerOperation,
    L,
>(
    by_length: &'a L,
    word: &'a impl HasWord,
) -> impl Iterator<Item = &'a <L as QueryableWordbucketList>::W>
where
    L: QueryableWordbucketList,
{
    let w: &Word = word.word();
    let target_length = (w.len() as i32 + Op::len_delta()) as usize;
    w.calc_letters()
        .operation_variations::<Op>()
        .flat_map(move |letter_mask| by_length.words_of_bucket(target_length, letter_mask))
        .filter(move |other| <Op as FilterWordsForOperation>::filter_for_operation(w, *other))
}

crate::base::operations::impl_operation_specific!(
    _filter_words_for_operation,
    pub trait FilterWordsForOperation: (Operation) {
        (
            fn filter_for_operation(start: &impl HasWord, target: &impl HasWord) -> bool,
                insert: {
                    only_one_extra_letter(start, target)
                },
                delete: {
                    only_one_extra_letter(target, start)
                },
                replace: {
                    only_one_different_letter(start, target)
                }
        )
    }
);
