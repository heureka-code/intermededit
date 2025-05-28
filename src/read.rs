use crate::AllWords;
use crate::MAX_WORD_LEN;
use crate::Word;
use crate::WordsOfLength;

pub fn read_wordlist(filename: &str) -> std::io::Result<AllWords> {
    use std::io::prelude::*;
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    let mut by_length = vec![WordsOfLength::new(); MAX_WORD_LEN + 2];

    for line in reader.lines() {
        let word = Word::new(&line?);
        assert!(word.len() < MAX_WORD_LEN);
        by_length[word.len()]
            .entry(word.calc_letters())
            .or_default()
            .push(word);
    }
    Ok(by_length)
}
