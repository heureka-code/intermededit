//! This module contains types for modeling the words of a list
//! and defines interfaces for organising them efficiently.
//!
//! # Layout of a [Word]
//! The words of a list are modelled in a special [Word] type.
//! This stores the word's content in uppercase as [`std::sync::Arc<str>`]
//! Words can generate an instance of [Letters] which describe
//! what letters occur in the word. If the `cache-letters` feature is enabled,
//! these instances will also be stored inside of the word.
//! By default this feature is turned of and the [Letters]
//! are computed for calling code when needed.
//!
//! Because there are only limited possibilities what could happen
//! to letter occurences in a word after a single
//! [operations::Insert], [operations::Replace] or [operations::Delete]
//! operation it is a good decision to group words by theese letters.
//!
//! This way words can be grouped in buckets depending on their [Letters]
//! and "near" words which aren't impossible to reach in one operation
//! can be accessed easier.
//!
//! ## Length of a [Word]
//! Typically the length of a [String] is it's number of bytes
//! so unicode characters would disturb the length grouping by bytes.
//! Thatswhy words count their length by iterating over [str::chars]
//! which yields individual characters and is aware of unicode cases.
//!
//! ## Purpose of a [TaggedWord]
//! Because some applications need to assign additional information to a word (an index)
//! wordlists require not a word but only a type implementing [HasWord].
//! This trait defines a method for getting a reference to a word
//! that can be used for i. e. [Letters] generation.
//!
//! # Generation of [Letters] for a word
//! To do this efficiently letters are "hashed" into bit positions
//! in a [u32] and a letter being present in a word at least once
//! activates the corresponding bit.
//!
//! There are multiple possible ways for assigning such bit masks
//! but for performance reasons a bitwise and with 31 (`0b11111`)
//! is used. Thisway only the least significant 5 bits of the numeric
//! value of each letter are used (this guarantees a range of 0--31).
//! This number can then be used to shift a `1u32` to the left.
//!
//! Due to the pattern in the values of ASCII characters
//! they all get individual bits and some unicode characters are grouped
//! with them. As most of the characters will be A--Z this is a good
//! distribution. Below table shows the different associations.
//!
//! The numeric values of the unicode characters seem to be "unicode scalars".
//! Anyhow they are the number assigned by a Rust [char].
//!
//!
//! | Bit | Characters          |   | Bit | Characters      |
//! | --- | ------------------- |---| --- | --------------- |
//! | 00  | Space (32), À (192) |   | 16  | P (80)          |
//! | 01  | A (65), Á (193)     |   | 17  | Q (81), Ñ (209) |
//! | 02  | B (66), Â (194)     |   | 18  | R (82), Ò (210) |
//! | 03  | C (67), Ã (195)     |   | 19  | S (83), Ó (211) |
//! | 04  | D (68), Ä (196)     |   | 20  | T (84), Ô (212) |
//! | 05  | E (69), Å (197)     |   | 21  | U (85)          |
//! | 06  | F (70), Æ (198)     |   | 22  | V (86), Ö (214) |
//! | 07  | G (71), Ç (199)     |   | 23  | W (87)          |
//! | 08  | H (72), È (200)     |   | 24  | X (88), Ø (216) |
//! | 09  | I (73), É (201)     |   | 25  | Y (89)          |
//! | 10  | J (74), Ê (202)     |   | 26  | Z (90), Ú (218) |
//! | 11  | K (75), Ë (203)     |   | 27  | Û (219)         |
//! | 12  | L (76), Ì (204)     |   | 28  | Ü (220)         |
//! | 13  | M (77), Í (205)     |   | 29  |                 |
//! | 14  | N (78), Î (206)     |   | 30  |                 |
//! | 15  | O (79), Ï (207)     |   | 31  |                 |
//!
//! (Characters occured in the used test file, grouped by their associated bits)
//!
//! # Organisation of wordlists
//! A wordlist should group words into buckets by their length and [Letters].
//!
//! Because there can be multiple possible implementation details a wordlist only needs
//! to implement some traits for being used as a wordlist by other modules.
//! Some interfaces require only some of them:
//!
//! - [QueryableWordbucketList]
//! - [InsertNewIntoWordbucketList]
//!
//!
//! When searching for all possible words that could be reached from a starting word
//! in one application of insert, replace or delete only some buckets must be
//! considered and others can be ignored entirely.
//!
//! ## Insert
//! The length increases by exactly one and at most one new bit can be set.
//! ([Letters::insert_variations])
//! The type marker that is used for generic code is [operations::Insert].
//!
//! ## Replace
//! The length stays the same and at most one new bit can be set and at most one
//! existing bit could unset.
//! ([Letters::substitution_variations])
//! The type marker that is used for generic code is [operations::Replace].
//!
//! ## Delete
//! The length decreases by one and at most one bit could be unset.
//! ([Letters::delete_variations])
//! The type marker that is used for generic code is [operations::Delete].

mod has_word;
mod letters;
mod tagged_word;
mod word;
mod wordlist_kinds;

mod operations_macro;
#[macro_use]
pub mod operations;

pub use has_word::HasWord;
pub use letters::{LetterVariationsPerOperation, Letters};
pub use tagged_word::TaggedWord;
pub use word::Word;
pub use wordlist_kinds::{InsertNewIntoWordbucketList, QueryableWordbucketList};

pub trait WordlistExt {
    type W: HasWord;
    fn remove_iter_from_words<'a>(&mut self, to_remove: impl Iterator<Item = &'a Self::W>)
    where
        Self::W: 'a;
}
