use std::ops::{Index, IndexMut};

/// Like `Vec` but grows as needed. Uninitialised elements in the tail have the given default value.
#[derive(Clone, Eq, Debug, Default)]
pub struct UnboundedVec<T: Clone> {
    initial: Vec<T>,
    default: T,
}

impl<T: Clone> UnboundedVec<T> {
    pub fn new(initial: Vec<T>, default: T) -> Self {
        UnboundedVec { initial, default }
    }
}

impl<T: Clone + PartialEq> PartialEq for UnboundedVec<T> {
    fn eq(&self, other: &Self) -> bool {
        // self.initial and other.initial may be different lengths but if the extra elements are all
        // default then self and other could still be equal.
        let max_len = self.initial.len().max(other.initial.len());
        (0..max_len).all(|i| self[i] == other[i]) && self.default == other.default
    }
}

impl<T: Clone> Index<usize> for UnboundedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if self.initial.len() <= index {
            &self.default
        } else {
            &self.initial[index]
        }
    }
}

impl<T: Clone> IndexMut<usize> for UnboundedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.initial.len() <= index {
            self.initial.resize(index + 1, self.default.clone());
        }
        &mut self.initial[index]
    }
}

#[macro_export]
macro_rules! unbounded_vec {
    ($default:expr) => {
        $crate::unbounded_vec::UnboundedVec::new(Vec::new(), $default)
    };
    ($($initial:expr),+; $default:expr) => {
        $crate::unbounded_vec::UnboundedVec::new(vec![$($initial),+], $default)
    };
}
