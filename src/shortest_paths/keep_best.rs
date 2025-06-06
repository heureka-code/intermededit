use num::Unsigned;

#[derive(derive_more::Deref, Debug)]
pub struct KeepBest<T, I>(#[deref] Vec<T>, I);

impl<T, I: Unsigned> Default for KeepBest<T, I> {
    fn default() -> Self {
        Self(vec![], I::zero())
    }
}

impl<T, I: Unsigned + Copy + PartialOrd> KeepBest<T, I> {
    pub fn new() -> Self {
        Self(vec![], I::zero())
    }
    pub fn current_max(&self) -> I {
        self.1
    }
    pub fn push(&mut self, value: I, item: T) {
        if value > self.1 {
            self.1 = value;
            self.0.clear();
        }
        if value == self.1 {
            self.0.push(item);
        }
    }
    pub fn extend(&mut self, value: I, items: impl Iterator<Item = T>) {
        if value > self.1 {
            self.1 = value;
            self.0.clear();
        }
        if value == self.1 {
            self.0.extend(items);
        }
    }
    pub fn take_vec(self) -> Vec<T> {
        self.0
    }
}
