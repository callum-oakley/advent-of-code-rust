use std::{collections::HashSet, hash::Hash};

pub struct Uniq<I: Iterator> {
    inner: I,
    seen: HashSet<I::Item>,
}

impl<I: Iterator> Uniq<I> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            seen: HashSet::new(),
        }
    }
}

impl<I: Iterator> Iterator for Uniq<I>
where
    I::Item: Eq,
    I::Item: Hash,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.inner.by_ref() {
            if !self.seen.contains(&item) {
                self.seen.insert(item.clone());
                return Some(item);
            }
        }
        None
    }
}
