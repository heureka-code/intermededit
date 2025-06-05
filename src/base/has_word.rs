use super::Word;

/// Trait that indicates that an implementing type contains a [Word].
/// This is implemented for [Word] and references of types that implement [HasWord].
pub trait HasWord {
    fn word(&self) -> &Word;
}
impl HasWord for Word {
    fn word(&self) -> &Word {
        self
    }
}
impl<T: HasWord> HasWord for &T {
    fn word(&self) -> &Word {
        (*self).word()
    }
}
