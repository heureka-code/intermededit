use crate::base::HasWord;

macro_rules! multi_impl {
    ($trait: ident for $($type: ty),* $(,)? { $bl: block }) => {
        $(
            impl $trait for $type {
                $bl
            }
        )*

    };
    ($trait: ident for $($type: ty),* $(,)?) => {
        $(
            impl $trait for $type { }
        )*

    };
}

pub trait OperationShortage {
    const NAME: &'static str;
}
macro_rules! impl_op_s {
    ($($ty: ty => $name: literal),+) => {
        $(
            impl OperationShortage for $ty {
                const NAME: &'static str = $name;
            }
        )+
    };
}

pub enum NoOps {}
pub enum Insert {}
pub enum Replace {}
pub enum Delete {}

pub enum InsertReplace {}
pub enum InsertDelete {}
pub enum ReplaceDelete {}
pub enum InsertReplaceDelete {}

impl_op_s! {
    NoOps => "___",
    Insert => "i__",
    Replace => "_r_",
    Delete => "__d",
    InsertReplace => "ir_",
    InsertDelete => "i_d",
    ReplaceDelete => "_rd",
    InsertReplaceDelete => "ird"
}

pub trait HasInsert {}
pub trait HasReplace {}
pub trait HasDelete {}

multi_impl!(HasInsert for Insert, InsertReplace, InsertDelete, InsertReplaceDelete);
multi_impl!(HasReplace for Replace, InsertReplace, ReplaceDelete, InsertReplaceDelete);
multi_impl!(HasDelete for Delete, InsertDelete, ReplaceDelete, InsertReplaceDelete);

pub(super) fn only_one_different_letter(first: &impl HasWord, second: &impl HasWord) -> bool {
    first
        .word()
        .chars()
        .zip(second.word().chars())
        .filter(|(a, b)| a != b)
        .count()
        == 1
}

pub(super) fn only_one_extra_letter(shorter: &impl HasWord, longer: &impl HasWord) -> bool {
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
