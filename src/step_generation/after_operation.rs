use super::check_for_change::{only_one_different_letter, only_one_extra_letter};
use crate::{
    Word,
    base::{HasWord, LetterVariationsPerOperation, QueryableWordbucketList, operations},
};

/// This function iterates over all words in a list that are reachable from a given word by
/// applying a specified operation.
///
/// The starting word is provided as second parameter.
/// Any type that implements [HasWord] can be this starting point,
/// the returned values will be of the type that the wordlist holds.
///
/// The operation to apply is specified by a type parameter of the function.
/// Possible values of this type are [operations::Insert], [operations::Replace] and
/// [operations::Delete]. All of them are only type markers can not be used to create values
pub fn find_after_operation<
    'a,
    Op: operations::Operation + FilterWordsForOperation + LetterVariationsPerOperation,
    L,
>(
    all_words: &'a L,
    word: &'a impl HasWord,
) -> impl Iterator<Item = &'a <L as QueryableWordbucketList>::W>
where
    L: QueryableWordbucketList,
{
    let w: &Word = word.word();
    let target_length = (w.len() as i32 + Op::len_delta()) as usize;
    w.calc_letters()
        .operation_variations::<Op>()
        .flat_map(move |letter_mask| all_words.words_of_bucket(target_length, letter_mask))
        .filter(move |other| <Op as FilterWordsForOperation>::filter_for_operation(w, *other))
}

crate::base::operations::impl_operation_specific!(
    _filter_words_for_operation,
    pub trait FilterWordsForOperation: (operations::Operation) {
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
