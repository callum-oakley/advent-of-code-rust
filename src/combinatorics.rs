pub struct Permutations<T> {
    current: Vec<T>,
    is_first: bool,
}

impl<T: Ord> Permutations<T> {
    pub fn new(mut values: Vec<T>) -> Self {
        values.sort_unstable();
        Self {
            current: values,
            is_first: true,
        }
    }
}

impl<T: Ord + Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    // https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            Some(self.current.clone())
        } else {
            (0..self.current.len() - 1)
                .rev()
                .find(|&i| self.current[i] < self.current[i + 1])
                .and_then(|i| {
                    (i + 1..self.current.len())
                        .rev()
                        .find(|&j| self.current[i] < self.current[j])
                        .map(|j| {
                            self.current.swap(i, j);
                            self.current[i + 1..].reverse();
                            self.current.clone()
                        })
                })
        }
    }
}
