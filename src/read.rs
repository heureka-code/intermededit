use crate::MAX_WORD_LEN;
use crate::Word;
use crate::base::InsertWordbucketList;

pub fn read_wordlist<L: InsertWordbucketList<Word> + Default>(
    filename: &str,
) -> std::io::Result<L> {
    use std::io::prelude::*;
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    let mut by_length = L::default();

    for line in reader.lines() {
        let word = Word::new(&line?);
        assert!(word.len() < MAX_WORD_LEN);
        by_length.insert_new(word);
    }
    Ok(by_length)
}
