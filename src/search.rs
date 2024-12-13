use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet, VecDeque},
    hash::Hash,
    ops::Add,
};

pub trait Queue {
    type Item;

    fn push(&mut self, value: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

/// Common or garden first-in-first-out queue.
impl<T> Queue for VecDeque<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) {
        VecDeque::push_back(self, value);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        VecDeque::pop_front(self)
    }
}

struct CostValue<V, O> {
    cost: O,
    value: V,
}

impl<V, O: PartialEq> PartialEq for CostValue<V, O> {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl<V, O: Eq> Eq for CostValue<V, O> {}

impl<V, O: PartialOrd> PartialOrd for CostValue<V, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl<V, O: Ord> Ord for CostValue<V, O> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

pub struct CostHeap<V, C, O> {
    cost: C,
    binary_heap: BinaryHeap<Reverse<CostValue<V, O>>>,
}

/// Priority queue which pops the lowest cost item first.
impl<V, C, O> Queue for CostHeap<V, C, O>
where
    C: FnMut(&V) -> O,
    O: Ord,
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

/// Traversal represents a graph traversal. The graph structure is implicit, it's up to the caller
/// to push adjacent states after each pop, but we'll remember states we've visited so that each
/// distinct state is popped at most once.
pub struct Traversal<Q, H, K> {
    queue: Q,
    hash_key: H,
    visited: HashSet<K>,
}

impl<Q, H, K> Queue for Traversal<Q, H, K>
where
    Q: Queue,
    H: FnMut(&Q::Item) -> K,
    K: Eq + Hash,
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
    K: Eq + Hash,
{
    let mut q = Traversal {
        queue,
        hash_key,
        visited: HashSet::new(),
    };
    q.push(start);
    q
}

/// Traverse a state space min-cost first. It's the caller's responsibility to push adjacent
/// states after each pop.
pub fn dijkstra<S, H, K, C, O>(start: S, hash_key: H, cost: C) -> impl Queue<Item = S>
where
    H: FnMut(&S) -> K,
    K: Eq + Hash,
    C: FnMut(&S) -> O,
    O: Ord,
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

/// Traverse a state space min-cost-plus-heuristic first. It's the caller's responsibility to push
/// adjacent states after each pop.
pub fn a_star<S, H, K, C, D, O>(
    start: S,
    hash_key: H,
    mut cost: C,
    mut heuristic: D,
) -> impl Queue<Item = S>
where
    H: FnMut(&S) -> K,
    K: Eq + Hash,
    C: FnMut(&S) -> O,
    D: FnMut(&S) -> O,
    O: Add,
    O::Output: Ord,
{
    dijkstra(start, hash_key, move |state| cost(state) + heuristic(state))
}
