use std::ops::{Index, IndexMut};

/// Like `Vec` but grows as needed. Elements are initialised to `T::default()`.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct UnboundedVec<T> {
    default: T,
    inner: Vec<T>,
}

impl<T: Default + Clone> UnboundedVec<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Default + Clone> FromIterator<T> for UnboundedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = UnboundedVec::new();
        for (i, x) in iter.into_iter().enumerate() {
            res[i] = x;
        }
        res
    }
}

impl<T: Default + Clone> Index<usize> for UnboundedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if self.inner.len() <= index {
            &self.default
        } else {
            &self.inner[index]
        }
    }
}

impl<T: Default + Clone> IndexMut<usize> for UnboundedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.inner.len() <= index {
            self.inner.resize(index + 1, T::default());
        }
        &mut self.inner[index]
    }
}
