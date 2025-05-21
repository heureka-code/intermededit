use super::{Word, AllWords};

fn only_one_extra_letter(shorter: &Word, longer: &Word) -> bool {
    let mut offset = 0;
    for (a, shortera) in shorter.chars().enumerate() {
        if Some(shortera) != longer.chars().nth(a + offset) {
            if offset == 0 {
                offset = 1;
                if Some(shortera) != longer.chars().nth(a + offset) {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn find_after_insertion<'a>(
    by_length: &'a AllWords,
    word: &'a Word,
) -> impl Iterator<Item = &'a Word> {
    let words_after_insertion = &by_length[word.len() + 1];
    word.calc_letters()
        .insert_variations()
        .flat_map(|letter_mask| words_after_insertion.get(&letter_mask))
        .flatten()
        .filter(move |other| only_one_extra_letter(&word, &other))
}

fn find_after_deletion<'a>(
    by_length: &'a AllWords,
    word: &'a Word,
) -> impl Iterator<Item = &'a Word> {
    let words_after_deletion = &by_length[word.len() - 1];
    word.calc_letters()
        .delete_variations()
        .flat_map(|letter_mask| words_after_deletion.get(&letter_mask))
        .flatten()
        .filter(move |other| only_one_extra_letter(&other, &word))
}

fn find_after_substitution<'a>(
    by_length: &'a AllWords,
    word: &'a Word,
) -> impl Iterator<Item = &'a Word> {
    let words_after_substitution = &by_length[word.len()];
    word.calc_letters()
        .substitution_variations()
        .flat_map(|letter_mask| words_after_substitution.get(&letter_mask))
        .flatten()
        .filter(move |other| {
            let mut diff = 0;
            for (a, b) in word.chars().zip(other.chars()) {
                if a != b {
                    diff = if diff == 0 { 1 } else { return false; };
                }
            }
            true
        })
}

pub fn all_after_one_step<'a>(
    by_length: &'a AllWords,
    word: &'a Word,
) -> impl Iterator<Item = &'a Word> {
    find_after_insertion(by_length, &word)
        .chain(find_after_deletion(by_length, &word))
        .chain(find_after_substitution(by_length, &word))
}


