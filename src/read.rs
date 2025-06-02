use crate::AllWords;
use crate::MAX_WORD_LEN;
use crate::Word;
use crate::base::LenLetWordlist;

pub fn read_wordlist(filename: &str) -> std::io::Result<AllWords> {
    use std::io::prelude::*;
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    let mut by_length = LenLetWordlist::default();

    for line in reader.lines() {
        let word = Word::new(&line?);
        assert!(word.len() < MAX_WORD_LEN);
        by_length.insert_new(word);
    }
    Ok(by_length)
}
