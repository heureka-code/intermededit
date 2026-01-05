use derive_more::{Debug, Display};
use std::ops::BitOr;

use crate::base::letters::Letters;

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[display("{lowercase}")]
#[debug("Word({lowercase:?})")]
pub struct Word<'a> {
    lowercase: &'a str,
}

/// Tagged word
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[debug("TWord({number:?}, {word:?})")]
pub struct TWord<'a, N> {
    word: Word<'a>,
    number: N,
}

impl<'a, N> super::HasWord for TWord<'a, N> {
    fn word(&self) -> &Word<'_> {
        &self.word
    }
}
impl<'a, N> TWord<'a, N> {
    pub fn tag(&self) -> &N {
        &self.number
    }
}

fn char_to_bit(lowercase_symbol: char) -> u32 {
    1u32 << ((lowercase_symbol as u32) & 0b0001_1111)
}

#[allow(clippy::len_without_is_empty)]
impl<'a> Word<'a> {
    pub fn number_with<N>(self, number: N) -> TWord<'a, N> {
        TWord { word: self, number }
    }
    pub fn new(lowercase: &'a str) -> Option<Self> {
        lowercase
            .chars()
            .all(|c| c.is_lowercase())
            .then_some(Self { lowercase })
    }
    pub fn len(&self) -> usize {
        self.lowercase.chars().count()
    }
    pub fn chars(&self) -> std::str::Chars<'_> {
        self.lowercase.chars()
    }
    pub fn calc_letters(&self) -> Letters {
        Letters::new(
            self.lowercase
                .chars()
                .map(char_to_bit)
                .fold(0, BitOr::bitor),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::base::word::Word;

    #[test]
    fn lowercase_ascii_ok() {
        let words = ["abc", "defghi", "something", "", "word", "nothing"];
        for w in words {
            assert!(Word::new(w).is_some());
        }
    }

    #[test]
    fn lowercase_unicode_ok() {
        let words = [
            "abäc",
            "defßghi",
            "äöü",
            "äöüßáà",
            "ŵ",
            "ê",
            "ẁ",
            "â",
            "éêâẁ",
        ];
        for w in words {
            assert!(Word::new(w).is_some());
        }
    }

    #[test]
    fn uppercase_ascii_err() {
        let words = ["aBc", "DefGhi", "AOUIE", "ZW d", "CAPS"];
        for w in words {
            assert!(Word::new(w).is_none());
        }
    }

    #[test]
    fn uppercase_unicode_err() {
        let words = ["aBÄÖc", "DefGhi", "AOUIEÀÁ", "ŴÊ", "ÒÓ"];
        for w in words {
            assert!(Word::new(w).is_none());
        }
    }
}
