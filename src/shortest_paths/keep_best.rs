use num::Unsigned;

#[derive(derive_more::Deref, Debug)]
pub struct KeepBest<T, I> {
    #[deref]
    best: Vec<T>,
    value: I,
}

impl<T, I: Unsigned> Default for KeepBest<T, I> {
    fn default() -> Self {
        Self {
            best: vec![],
            value: I::zero(),
        }
    }
}

impl<T, I: Unsigned + Copy + PartialOrd> KeepBest<T, I> {
    pub fn new() -> Self {
        Self {
            best: vec![],
            value: I::zero(),
        }
    }
    pub fn current_max(&self) -> I {
        self.value
    }
    pub fn push(&mut self, value: I, item: T) {
        if self.should_use_argument(value) {
            self.best.push(item);
        }
    }
    pub fn extend(&mut self, value: I, items: impl Iterator<Item = T>) {
        if self.should_use_argument(value) {
            self.best.extend(items);
        }
    }
    pub fn take_vec(self) -> Vec<T> {
        self.best
    }
    fn should_use_argument(&mut self, value: I) -> bool {
        if value > self.value {
            self.value = value;
            self.best.clear();
        }
        value == self.value
    }
    pub fn best(&self) -> impl Iterator<Item = &T> {
        self.best.iter()
    }
}

impl<T, I> IntoIterator for KeepBest<T, I> {
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.best.into_iter()
    }
}
