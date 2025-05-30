use super::letters::Letters;
use derive_more::{Display, From, Into};
use std::sync::Arc;

/// Represents a single word in uppercase letters.
///
/// Internally it stores the text as [`Arc<str>`] so it's cheap to copy and thread safe.
///
/// If the `cache-letters` feature is enabled it will also contain the matching instance of [Letters].
/// If the feature is disabled (default), the instance will be computed every time it is requested.
///
/// This implementation assigns each letter a bit by considering only the least significant five
/// bits and uses them for shifting 1 one as [`u32`].
///
#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, From, Into)]
#[display("{uppercase}")]
pub struct Word {
    uppercase: Arc<str>,
    #[cfg(feature = "cache-letters")]
    letters: Letters,
}

#[allow(clippy::len_without_is_empty)]
impl Word {
    /// Iterator over the single characters ([str::chars])
    pub fn chars(&self) -> impl Iterator<Item = char> {
        self.uppercase.chars()
    }
    #[cfg(feature = "unicode-word-len")]
    /// The length of the word, if the feature `unicode-word-len` is enabled (default) this is
    /// the number of characters yielded by [str::chars], otherwise it's simply the number of bytes
    pub fn len(&self) -> usize {
        self.chars().count()
    }
    #[cfg(not(feature = "unicode-word-len"))]
    /// The length of the word, if the feature `unicode-word-len` is enabled (default) this is
    /// the number of characters yielded by [str::chars], otherwise it's simply the number of bytes
    pub fn len(&self) -> usize {
        self.uppercase.len()
    }
}

fn letters_from_uppercase(uppercase: &str) -> Letters {
    let mut val = 0;
    for n in uppercase.chars().map(|c| c as u32) {
        // println!("{:02} {c} {n}", n & 0b0001_1111);

        // looks like the most efficient and optimal calculation for the occuring characters
        val |= 1u32 << (n & 0b0001_1111);

        /*
        // This transforms A-Z to 0-25 and shifts, other characters are difficult to deal with
        val |= 1u32.wrapping_shl((n as u8).wrapping_sub(b'A') as u32);

        // This transforms A-Z and some diacritic characters with few collisions into a 64-bit-mask
        val |= 1u64 | ((n & 0b0011_1111) | ((n & 128) >> 2));

        // This also deactivates bit with value 32 maybe better as 128 is moved to this position
        val |= 1u64 << ((n & 0b0001_1111) | ((n & 128) >> 2));
        */
    }
    Letters::new(val)
}

#[cfg(not(feature = "cache-letters"))]
impl Word {
    /// If the `cache-letters` feature is enabled this will return the copy of a field that was
    /// created by [Word::new].
    /// If the feature is disabled (default), the instance will be computed every time this method runs.
    pub fn calc_letters(&self) -> Letters {
        letters_from_uppercase(&self.uppercase)
    }
    /// Turns the provided string slice into uppercase and stores the resulting string in and
    /// [`Arc<str>`]
    ///
    /// If the `cache-letters` feature is enabled it will also compute a [Letters] instance for
    /// this word.
    /// If the feature is disabled (default), the type will only contain the text and the bit mask
    /// gets computed on every invocation of [Self::calc_letters].
    pub fn new(text: &str) -> Self {
        // NOTE: maybe extra feature flag for to_ascii_uppercase()
        Self {
            uppercase: text.to_uppercase().into(),
        }
    }
}

#[cfg(feature = "cache-letters")]
impl Word {
    /// If the `cache-letters` feature is enabled this will return the copy of a field that was
    /// created by [Word::new].
    /// If the feature is disabled (default), the instance will be computed every time this method runs.
    pub fn calc_letters(&self) -> Letters {
        self.letters
    }
    /// Turns the provided string slice into uppercase and stores the resulting string in and
    /// [`Arc<str>`]
    ///
    /// If the `cache-letters` feature is enabled it will also compute a [Letters] instance for
    /// this word.
    /// If the feature is disabled (default), the type will only contain the text and the bit mask
    /// gets computed on every invocation of [Self::calc_letters].
    pub fn new(text: &str) -> Self {
        // NOTE: maybe extra feature flag for to_ascii_uppercase()
        let uppercase: Arc<str> = text.to_uppercase().into();
        Self {
            letters: letters_from_uppercase(&uppercase),
            uppercase,
        }
    }
}
