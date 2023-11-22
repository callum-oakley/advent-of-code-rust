use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    hash::Hash,
    ops::Add,
};

pub trait Queue {
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

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct CostValue<V, O> {
    cost: O,
    value: V,
}

pub struct CostHeap<V, C, O> {
    cost: C,
    binary_heap: BinaryHeap<Reverse<CostValue<V, O>>>,
}

// Priority queue which pops the lowest cost item first.
impl<V, C, O> Queue for CostHeap<V, C, O>
where
    V: PartialOrd + Ord + PartialEq + Eq,
    C: FnMut(&V) -> O,
    O: PartialOrd + Ord + PartialEq + Eq,
{
    type Item = V;

    fn push(&mut self, value: Self::Item) {
        self.binary_heap.push(Reverse(CostValue {
            cost: (self.cost)(&value),
            value,
        }));
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.binary_heap.pop().map(|w| w.0.value)
    }
}

pub struct Traversal<Q, H, K> {
    queue: Q,
    hash_key: H,
    visited: HashSet<K>,
}

impl<Q, H, K> Queue for Traversal<Q, H, K>
where
    Q: Queue,
    H: FnMut(&Q::Item) -> K,
    K: PartialEq + Eq + Hash,
{
    type Item = Q::Item;

    fn push(&mut self, state: Self::Item) {
        if !self.visited.contains(&(self.hash_key)(&state)) {
            self.queue.push(state);
        }
    }

    fn pop(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.queue.pop() {
            let key = (self.hash_key)(&state);
            if !self.visited.contains(&key) {
                self.visited.insert(key);
                return Some(state);
            }
        }
        None
    }
}

fn traverse<Q, S, H, K>(queue: Q, start: S, hash_key: H) -> impl Queue<Item = S>
where
    Q: Queue<Item = S>,
    H: FnMut(&S) -> K,
    K: PartialEq + Eq + Hash,
{
    let mut q = Traversal {
        queue,
        hash_key,
        visited: HashSet::new(),
    };
    q.push(start);
    q
}

pub fn breadth_first<S, H, K>(start: S, hash_key: H) -> impl Queue<Item = S>
where
    H: FnMut(&S) -> K,
    K: PartialEq + Eq + Hash,
{
    traverse(VecDeque::new(), start, hash_key)
}

// pub fn depth_first<S, H, K>(start: S, hash_key: H) -> impl Queue<Item = S>
// where
//     H: FnMut(&S) -> K,
//     K: PartialEq + Eq + Hash,
// {
//     traverse(Vec::new(), start, hash_key)
// }

pub fn dijkstra<S, H, K, C, O>(start: S, hash_key: H, cost: C) -> impl Queue<Item = S>
where
    H: FnMut(&S) -> K,
    K: PartialEq + Eq + Hash,
    S: PartialOrd + Ord + PartialEq + Eq,
    C: FnMut(&S) -> O,
    O: PartialOrd + Ord + PartialEq + Eq,
{
    traverse(
        CostHeap {
            cost,
            binary_heap: BinaryHeap::new(),
        },
        start,
        hash_key,
    )
}

pub fn a_star<S, H, K, C, D, O>(
    start: S,
    hash_key: H,
    mut cost: C,
    mut heuristic: D,
) -> impl Queue<Item = S>
where
    H: FnMut(&S) -> K,
    K: PartialEq + Eq + Hash,
    S: PartialOrd + Ord + PartialEq + Eq,
    C: FnMut(&S) -> O,
    D: FnMut(&S) -> O,
    O: PartialOrd + Ord + PartialEq + Eq + Add,
    O::Output: Ord,
{
    dijkstra(start, hash_key, move |state| cost(state) + heuristic(state))
}
