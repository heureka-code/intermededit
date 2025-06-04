pub(crate) use super::operations_macro::impl_operation_specific;

pub enum Insert {}
pub enum Replace {}
pub enum Delete {}

impl_operation_specific!(
    operation_trait,
    /// This trait defines editing operations on a word and should only be implemented
    /// for [Insert], [Replace] and [Delete]
    pub trait Operation {
        (
            /// returns a static string literal naming the operation.
            fn lowercase_label() -> &'static str,
                /// `"insert"`
                insert: { "insert" },
                /// `"delete"`
                delete: { "delete" },
                /// `"replace"`
                replace: { "replace" }
        )
        (
            /// returns the difference this operation causes in the length of a word when applied
            fn len_delta() -> i32,
                /// increases the length by one -> `+1`
                insert: { 1 },
                /// decreases the length by one -> `-1`
                delete: { -1 },
                /// doesn't change the length -> `0`
                replace: { 0 }
        )
    }
);
