// https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
// Mutates the input into the lexicographically next permutation. Returns false
// if we've reached the end.
pub fn permute<T: PartialOrd>(xs: &mut [T]) -> bool {
    if let Some(i) = (0..xs.len() - 1).rev().find(|&i| xs[i] < xs[i + 1]) {
        if let Some(j) = (i + 1..xs.len()).rev().find(|&j| xs[i] < xs[j]) {
            xs.swap(i, j);
            xs[i + 1..].reverse();
            return true;
        }
    }
    false
}
