use derive_more::{From, Into};
use itertools::Itertools;

/// Represents a bit mask associated with a word.
/// When a value of this type is generated for a word every character in the word gets
/// associated with a bit in a [u32] and the bitwise or of all of these bits is the stored mask.
///
/// With this it is possible to group words by the letters they contain.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, From, Into)]
pub struct Letters(u32);

impl Letters {
    pub(super) fn new(mask: u32) -> Self {
        Self(mask)
    }
    /// Helper function for getting the cartesian product of
    /// the set of powers of two (extended with 0) with itself.
    ///
    /// This means we can iterate over all pairs of bits that can be set in a [u32] bit mask.
    /// *The only pair that is not yielded is (0, 0) as it's not needed by the other methods
    ///  and their implementation doesn't require this pair to be present.*
    fn power_product() -> impl Iterator<Item = (u32, u32)> {
        POWERS_OF_TWO
            .into_iter()
            .cartesian_product(POWERS_OF_TWO)
            .chain(POWERS_OF_TWO.into_iter().map(|p| (0, p)))
            .chain(POWERS_OF_TWO.into_iter().map(|p| (p, 0)))
    }

    /// Iterator over all [Letters] bit masks which could be generated when inserting a single
    /// letter into the associated word. This is achieved by iterating over the 32 representable bits
    /// and yielding the bitwise or of the stored mask and the current bit.
    ///
    /// If a new letter is added it's bit value could be already in the mask
    /// (when a letter with the same associated bit value is already in the word)
    ///
    /// or no letter with this letter's associated bit is already in the mask and one new bit must
    /// be set.
    ///
    /// Both cases are considered by this method, it yields at most 32 such masks without
    /// duplicates.
    pub fn insert_variations(self) -> impl Iterator<Item = Letters> {
        POWERS_OF_TWO
            .iter()
            .map(move |bit| Letters(self.0 | bit))
            .unique()
    }
    /// Iterator over all [Letters] bit masks which could be generated when deleting a single
    /// letter from the associated word. This is achieved by iterating over the 32 representable bits (and 0)
    /// and yielding the bitwise and of the stored mask with the bitwise negation of the current bit.
    ///
    /// If a letter is removed whose bit value is different from all other letter's associated bits
    /// this bit needs to be deactivated.
    ///
    /// If there is another letter with this associated bit nothing should change. This is achieved
    /// by including 0 in the iterator of "bits".
    ///
    /// This method yields at most 33 such masks without duplicates.
    pub fn delete_variations(self) -> impl Iterator<Item = Letters> {
        POWERS_OF_TWO
            .iter()
            .chain(std::iter::once(&0u32))
            .map(move |bit| Letters(self.0 & !bit))
            .unique()
    }
    /// Iterator over all [Letters] bit masks which could be generated when replacing a single
    /// letter in the associated word.
    /// This is achieved by iterating over all possible combinations (cartesian product)
    /// of the 32 representable bits and 0 (see [Self::delete_variations])
    /// and yielding the result of bitwise operations which activate one bit and deactivate the
    /// other.
    ///
    /// This method yields no duplicates.
    pub fn substitution_variations(self) -> impl Iterator<Item = Letters> {
        Self::power_product()
            .map(move |(inserted, deleted)| Letters((self.0 & !deleted) | inserted))
            .unique()
    }
}

/// Array of powers of two that fit into a [u32].
/// This is used for iterating over all bits that could be set in a bit mask of [Letters]
pub const POWERS_OF_TWO: [u32; 32] = [
    1 << 31,
    1 << 30,
    1 << 29,
    1 << 28,
    1 << 27,
    1 << 26,
    1 << 25,
    1 << 24,
    1 << 23,
    1 << 22,
    1 << 21,
    1 << 20,
    1 << 19,
    1 << 18,
    1 << 17,
    1 << 16,
    1 << 15,
    1 << 14,
    1 << 13,
    1 << 12,
    1 << 11,
    1 << 10,
    1 << 9,
    1 << 8,
    1 << 7,
    1 << 6,
    1 << 5,
    1 << 4,
    1 << 3,
    1 << 2,
    1 << 1,
    1 << 0,
];
