use crate::base::HasWord;

pub(super) fn only_one_different_letter(first: &impl HasWord, second: &impl HasWord) -> bool {
    first
        .word()
        .chars()
        .zip(second.word().chars())
        .filter(|(a, b)| a != b)
        .count()
        < 2
}

pub(super) fn only_one_extra_letter(shorter: &impl HasWord, longer: &impl HasWord) -> bool {
    let mut offset = false;
    let mut longer_chars = longer.word().chars();
    for shortera in shorter.word().chars() {
        if Some(shortera) != longer_chars.next() {
            if !offset {
                offset = true;
                if Some(shortera) == longer_chars.next() {
                    continue;
                }
            }
            return false;
        }
    }
    true
}
