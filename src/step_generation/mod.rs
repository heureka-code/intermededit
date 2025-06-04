mod after_operation;
mod check_for_change;

use crate::base::{HasWord, QueryableWordbucketList};

pub use after_operation::{FilterWordsForOperation, find_after_operation};

pub fn all_after_one_step<'a, L>(
    by_length: &'a L,
    word: &'a impl HasWord,
) -> impl Iterator<Item = &'a <L as QueryableWordbucketList>::W>
where
    L: QueryableWordbucketList,
{
    use crate::base::operations::{Delete, Insert, Replace};
    find_after_operation::<Insert, L>(by_length, word)
        .chain(find_after_operation::<Delete, L>(by_length, word))
        .chain(find_after_operation::<Replace, L>(by_length, word))
}
