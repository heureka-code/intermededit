use super::{HasWord, Letters};

pub trait QueryableWordbucketList {
    type W: HasWord;

    fn words_of_bucket(&self, len: usize, letters: Letters) -> impl Iterator<Item = &Self::W>;
    fn iter_all(&self) -> impl Iterator<Item = &Self::W>;

    fn get_any_word(&self) -> Option<&Self::W> {
        self.iter_all().next()
    }
    fn get_word_count(&self) -> usize {
        self.iter_all().count()
    }
}

pub trait InsertWordbucketList<Insertable> {
    fn insert_new(&mut self, word: Insertable);
}
