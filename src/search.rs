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

// Common or garden first-in-first-out queue.
impl<T> Queue for VecDeque<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        VecDeque::push_back(self, value);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        VecDeque::pop_front(self)
    }
}

// Last-in-first-out "queue"... in other words a stack.
impl<T> Queue for Vec<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        Vec::push(self, value);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        Vec::pop(self)
    }
}

// Priority queue which pops the minimum item. By default BinaryHeap is a
// max-heap so Reverse.
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
    type Adjacent;

    fn adjacent(&self) -> Self::Adjacent;
}

pub struct Traversal<S: State, Q> {
    queue: Q,
    visited: HashSet<S>,
}

impl<S, Q> Iterator for Traversal<S, Q>
where
    S: State + Eq + Hash + Clone,
    S::Adjacent: IntoIterator<Item = S>,
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
pub fn breadth_first<S: State>(start: S) -> Traversal<S, VecDeque<S>> {
    Traversal {
        queue: VecDeque::from([start]),
        visited: HashSet::new(),
    }
}

/// Traverse the state space depth first. Implement Hash and Eq on S to specify
/// when two states should be considered the same.
#[allow(dead_code)]
pub fn depth_first<S: State>(start: S) -> Traversal<S, Vec<S>> {
    Traversal {
        queue: Vec::from([start]),
        visited: HashSet::new(),
    }
}

/// Traverse the state space in increasing order according to Ord. If Ord
/// represents an ordering by a cost function then this is Dijkstra's algorithm.
/// If it also factors in a heuristic then this is A*. Implement Hash and Eq on
/// S to specify when two states should be considered the same.
pub fn min_first<S: State + Ord>(start: S) -> Traversal<S, BinaryHeap<Reverse<S>>> {
    Traversal {
        queue: BinaryHeap::from([Reverse(start)]),
        visited: HashSet::new(),
    }
}
