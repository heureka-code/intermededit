use crate::base::{Word, operations as op, wordlist_ext::QueryableWordbucketList};

fn find_after_insert<'a, L>(
    all_words: &'a L,
    word: &'a Word,
) -> impl Iterator<Item = &'a <L as QueryableWordbucketList<'a>>::W>
where
    L: QueryableWordbucketList<'a>,
{
    let target_length = word.len() + 1;
    word.calc_letters()
        .insert_variations()
        .flat_map(move |letter_mask| all_words.words_of_bucket(target_length, letter_mask))
        .filter(move |candidate| op::only_one_extra_letter(word, candidate))
}

fn find_after_replace<'a, L>(
    all_words: &'a L,
    word: &'a Word,
) -> impl Iterator<Item = &'a <L as QueryableWordbucketList<'a>>::W>
where
    L: QueryableWordbucketList<'a>,
{
    let target_length = word.len();
    word.calc_letters()
        .substitution_variations()
        .flat_map(move |letter_mask| all_words.words_of_bucket(target_length, letter_mask))
        .filter(move |candidate| op::only_one_different_letter(word, candidate))
}

fn find_after_delete<'a, L>(
    all_words: &'a L,
    word: &'a Word,
) -> impl Iterator<Item = &'a <L as QueryableWordbucketList<'a>>::W>
where
    L: QueryableWordbucketList<'a>,
{
    let target_length = word.len() - 1; // WARNING: panics if length is 0
    word.calc_letters()
        .delete_variations()
        .flat_map(move |letter_mask| all_words.words_of_bucket(target_length, letter_mask))
        .filter(move |candidate| op::only_one_extra_letter(candidate, word))
}

pub trait FindAfterOperation {
    fn find_after_operation<'a, L>(
        all_words: &'a L,
        word: &'a Word,
    ) -> impl Iterator<Item = &'a <L as QueryableWordbucketList<'a>>::W>
    where
        L: QueryableWordbucketList<'a>;
}
macro_rules! impl_find_after {
    ($($type: ty : $($atomOp: ident)|+),* $(,)?) => {
        $(
        impl FindAfterOperation for $type {
            fn find_after_operation<'a, L>(
                all_words: &'a L,
                word: &'a Word,
            ) -> impl Iterator<Item = &'a <L as QueryableWordbucketList<'a>>::W>
            where
                L: QueryableWordbucketList<'a>,
            {
                std::iter::empty()
                    $(.chain(impl_find_after!(; $atomOp; all_words, word)))*
            }
        }
        )*
    };
    (; insert; $a: ident, $w: ident) => {
        find_after_insert($a, $w)
    };
    (; replace; $a: ident, $w: ident) => {
        find_after_replace($a, $w)
    };
    (; delete; $a: ident, $w: ident) => {
        find_after_delete($a, $w)
    };
}

impl_find_after!(
    op::Insert: insert,
    op::Replace: replace,
    op::Delete: delete,
    op::InsertReplace: insert | replace,
    op::InsertDelete: insert | delete,
    op::ReplaceDelete: replace | delete,
    op::InsertReplaceDelete: insert | replace | delete,
);
