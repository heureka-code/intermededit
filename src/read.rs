use crate::MAX_WORD_LEN;
use crate::Word;
use crate::base::InsertNewIntoWordbucketList;

pub fn read_wordlist<L: InsertNewIntoWordbucketList<Word> + Default>(
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

pub fn wordlist_path() -> String {
    std::env::var("WORDLIST_FILE").unwrap_or(crate::DEFAULT_WORDLIST.to_string())
}
pub fn expect_wordlist<L: InsertNewIntoWordbucketList<Word> + Default>() -> L {
    read_wordlist(&wordlist_path()).expect("Wordlist file")
}
