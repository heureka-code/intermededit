use super::Word;

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
