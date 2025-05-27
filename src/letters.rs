use derive_more::{Deref, From, Into};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deref, From, Into)]
pub struct Letters(pub u32);

impl Letters {
    fn power_product() -> impl Iterator<Item = (u32, u32)> {
        POWERS_OF_TWO
            .into_iter()
            .cartesian_product(POWERS_OF_TWO)
            .chain(POWERS_OF_TWO.into_iter().map(|p| (0, p)))
            .chain(POWERS_OF_TWO.into_iter().map(|p| (p, 0)))
    }

    pub fn insert_variations(self) -> impl Iterator<Item = Letters> {
        POWERS_OF_TWO
            .iter()
            .map(move |bit| Letters(self.0 | bit))
            .unique()
    }
    pub fn delete_variations(self) -> impl Iterator<Item = Letters> {
        POWERS_OF_TWO
            .iter()
            .chain(std::iter::once(&0u32))
            .map(move |bit| Letters(self.0 & !bit))
            .unique()
    }
    pub fn substitution_variations(self) -> impl Iterator<Item = Letters> {
        Self::power_product().map(move |(inserted, deleted)| {
            Letters((self.0 & !deleted) | inserted)
        })
            .unique()
    }
}

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
