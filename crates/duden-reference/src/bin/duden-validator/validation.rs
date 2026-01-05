// UNUSED

use std::{collections::HashSet, hash::Hash};

use duden_reference::{FindAfterOperation, HasWord, TWord};
use itertools::Itertools;

pub fn find_shortest_path_max<O: FindAfterOperation, N: Eq + Hash>(
    pack: &Pack<'_, N>,
    max_steps: usize,
    from: &TWord<'_, N>,
    to: &TWord<'_, N>,
) -> Result<(), WayValidationErr<'static>> {
    let mut current: HashSet<&TWord<'_, N>> = HashSet::from_iter([from]);

    for iteration in 0..max_steps {
        if current.contains(to) {
            return Err(WayValidationErr::TooLong {
                shortest: iteration,
            });
        }
        let mut new = HashSet::new();
        for c in current.drain() {
            new.extend(O::find_after_operation(pack.list, c.word()));
        }
        std::mem::swap(&mut new, &mut current);
    }
    if current.contains(to) {
        Ok(())
    } else {
        Err(WayValidationErr::ShorterThanPossible)
    }
}

pub fn validate_way<'a, O: FindAfterOperation, N>(
    pack: &Pack<'_, N>,
    way: &[&'a str],
) -> Result<(), WayValidationErr<'a>>
where
    N: PartialEq + Eq + Hash,
{
    if way.len() <= 1 {
        return Ok(());
    }
    let mut steps = Vec::with_capacity(way.len());
    for (idx, word) in way.iter().enumerate() {
        if let Some(nw) = pack.lookup.get(word) {
            steps.push(nw);
        } else {
            return Err(WayValidationErr::NonExistantWord(word));
        }
    }
    for (idx, (from, to)) in steps.iter().tuple_windows().enumerate() {
        if from.word().len().abs_diff(to.word().len()) > 1 {
            return Err(WayValidationErr::InvalidStep(idx));
        }
        let valid = O::find_after_operation(pack.list, from.word()).contains(to);
        if !valid {
            return Err(WayValidationErr::InvalidStep(idx));
        }
    }
    find_shortest_path_max::<O, N>(
        pack,
        way.len() - 1,
        *steps.first().unwrap(),
        *steps.last().unwrap(),
    )
}
/*
    // let way = vec!["bier", "eier", "eber", "leber"];
    let way = ["herz", "harz", "hart", "hast", "hase", "hasen", "rasen"];
    let res = validate_way::<, u32>(&pack, &way);
    println!("{res:?}");
*/
