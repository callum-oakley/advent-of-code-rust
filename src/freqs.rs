use std::{collections::HashMap, hash::Hash};

pub trait Freqs: Iterator {
    fn freqs(self) -> HashMap<Self::Item, usize>;
}

impl<T> Freqs for T
where
    T: Iterator,
    T::Item: Eq + Hash,
{
    fn freqs(self) -> HashMap<Self::Item, usize> {
        let mut res = HashMap::new();
        for x in self {
            *res.entry(x).or_default() += 1;
        }
        res
    }
}
