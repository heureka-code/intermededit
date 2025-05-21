use super::letters::Letters;
use derive_more::{Deref, From, Into, Display};
use std::sync::Arc;

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deref, From, Into)]
pub struct Word(Arc<str>);

fn letters_from_uppercase(uppercase: &str) -> Letters {
    let mut val = 0;
    for c in uppercase.chars() {
        val |= 1u32 << (c as u8 - 'A' as u8);
    }
    Letters(val)
}

impl Word {
    pub fn calc_letters(&self) -> Letters {
        letters_from_uppercase(&self.0)
    }
    pub fn new(text: &str) -> Self {
        Self(text.to_ascii_uppercase().into())
    }
}
