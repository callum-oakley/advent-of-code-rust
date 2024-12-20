use std::{collections::HashMap, hash::Hash};

pub trait Freq: Iterator {
    fn freq(self) -> HashMap<Self::Item, usize>;
}

impl<T> Freq for T
where
    T: Iterator,
    T::Item: Eq + Hash,
{
    fn freq(self) -> HashMap<Self::Item, usize> {
        let mut res = HashMap::new();
        for x in self {
            *res.entry(x).or_default() += 1;
        }
        res
    }
}
