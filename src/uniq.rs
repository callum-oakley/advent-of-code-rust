use std::{collections::HashSet, hash::Hash, iter};

pub trait Uniq: Iterator {
    fn uniq(self) -> impl Iterator<Item = Self::Item>;
}

impl<T> Uniq for T
where
    T: Iterator,
    T::Item: Clone + Eq + Hash,
{
    fn uniq(mut self) -> impl Iterator<Item = Self::Item> {
        let mut seen: HashSet<Self::Item> = HashSet::new();
        iter::from_fn(move || {
            for item in self.by_ref() {
                if !seen.contains(&item) {
                    seen.insert(item.clone());
                    return Some(item);
                }
            }
            None
        })
    }
}
