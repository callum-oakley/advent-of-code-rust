use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    hash::Hash,
};

trait Queue {
    type Item;

    fn push(&mut self, value: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

impl<T> Queue for VecDeque<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        VecDeque::push_back(self, value);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        VecDeque::pop_front(self)
    }
}

impl<T> Queue for Vec<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        Vec::push(self, value);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        Vec::pop(self)
    }
}

// By default BinaryHeap is a max-heap. We need a min-heap so Reverse.
impl<T: Ord> Queue for BinaryHeap<Reverse<T>> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        BinaryHeap::push(self, Reverse(value));
    }

    fn pop(&mut self) -> Option<Self::Item> {
        BinaryHeap::pop(self).map(|value| value.0)
    }
}

pub trait State {
    // TODO it would be nice if adjacent could return an IntoIterator
    fn adjacent(&self) -> Vec<Self>
    where
        Self: Sized;
}

pub struct Traversal<S, Q> {
    queue: Q,
    visited: HashSet<S>,
}

impl<S, Q> Iterator for Traversal<S, Q>
where
    S: State + Eq + Hash + Clone,
    Q: Queue<Item = S>,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.queue.pop() {
            if !self.visited.contains(&state) {
                self.visited.insert(state.clone());
                for s in state.adjacent() {
                    if !self.visited.contains(&s) {
                        self.queue.push(s);
                    }
                }
                return Some(state);
            }
        }
        None
    }
}

/// Traverse the state space breadth first. Implement Hash and Eq on S to
/// specify when two states should be considered the same.
#[allow(dead_code)]
pub fn breadth_first<S>(start: S) -> Traversal<S, VecDeque<S>> {
    Traversal {
        queue: VecDeque::from([start]),
        visited: HashSet::new(),
    }
}

/// Traverse the state space depth first. Implement Hash and Eq on S to specify
/// when two states should be considered the same.
#[allow(dead_code)]
pub fn depth_first<S>(start: S) -> Traversal<S, Vec<S>> {
    Traversal {
        queue: Vec::from([start]),
        visited: HashSet::new(),
    }
}

/// Traverse the state space in increasing order according to Ord. If Ord
/// represents an ordering by a cost function then this is Dijkstra's algorithm.
/// If it also factors in a heuristic then this is A*. Implement Hash and Eq on
/// S to specify when two states should be considered the same.
pub fn min_first<S: Ord>(start: S) -> Traversal<S, BinaryHeap<Reverse<S>>> {
    Traversal {
        queue: BinaryHeap::from([Reverse(start)]),
        visited: HashSet::new(),
    }
}
