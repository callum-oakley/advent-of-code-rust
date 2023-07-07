use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet, VecDeque},
    hash::{Hash, Hasher},
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

/// A state in a searchable state space. If we can move to adjacent states, and
/// we can hash a state to see if we've visited it already, then we can traverse
/// the corresponding space. `StateWrapper` will implement `PartialEq`, `Eq`,
/// and `Hash` based on `hash_key` such that two states with the same `hash_key`
/// will be considered equal.
pub trait State {
    type HashKey;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_>;
    fn hash_key(&self) -> Self::HashKey;
}

pub struct StateWrapper<S>(S);

impl<S> PartialEq for StateWrapper<S>
where
    S: State,
    <S as State>::HashKey: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.hash_key() == other.0.hash_key()
    }
}

impl<S> Eq for StateWrapper<S>
where
    S: State,
    <S as State>::HashKey: Eq,
{
}

impl<S> Hash for StateWrapper<S>
where
    S: State,
    <S as State>::HashKey: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash_key().hash(state);
    }
}

/// Represent an ordering based on a key, e.g. a cost or a cost + a heuristic.
/// `StateWrapper` will implement `Ord` and `PartialOrd` based on this.
pub trait OrdKey {
    type OrdKey;

    fn ord_key(&self) -> Self::OrdKey;
}

impl<S> PartialOrd for StateWrapper<S>
where
    S: State + OrdKey,
    <S as State>::HashKey: PartialEq,
    <S as OrdKey>::OrdKey: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.ord_key().partial_cmp(&other.0.ord_key())
    }
}

impl<S> Ord for StateWrapper<S>
where
    S: State + OrdKey,
    <S as State>::HashKey: Eq,
    <S as OrdKey>::OrdKey: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.ord_key().cmp(&other.0.ord_key())
    }
}

pub struct Traversal<S, Q> {
    queue: Q,
    visited: HashSet<StateWrapper<S>>,
}

impl<S, Q> Iterator for Traversal<S, Q>
where
    S: Clone + State,
    <S as State>::HashKey: Hash + Eq,
    Q: Queue<Item = StateWrapper<S>>,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.queue.pop() {
            if !self.visited.contains(&state) {
                self.visited.insert(StateWrapper(state.0.clone()));
                for s in state.0.adjacent() {
                    let s = StateWrapper(s);
                    if !self.visited.contains(&s) {
                        self.queue.push(s);
                    }
                }
                return Some(state.0);
            }
        }
        None
    }
}

pub type BreadthFirstTraversal<S> = Traversal<S, VecDeque<StateWrapper<S>>>;

/// Traverse the state space breadth first.
pub fn breadth_first<S>(start: S) -> BreadthFirstTraversal<S>
where
    S: State,
{
    Traversal {
        queue: VecDeque::from([StateWrapper(start)]),
        visited: HashSet::new(),
    }
}

pub type DepthFirstTraversal<S> = Traversal<S, Vec<StateWrapper<S>>>;

/// Traverse the state space depth first.
pub fn depth_first<S>(start: S) -> DepthFirstTraversal<S>
where
    S: State,
{
    Traversal {
        queue: Vec::from([StateWrapper(start)]),
        visited: HashSet::new(),
    }
}

pub type MinFirstTraversal<S> = Traversal<S, BinaryHeap<Reverse<StateWrapper<S>>>>;

/// Traverse the state space in increasing order of `ord_key`. If `ord_key`
/// represents an ordering by a cost function then this is Dijkstra's algorithm.
/// If it also factors in a heuristic then this is A*.
pub fn min_first<S>(start: S) -> MinFirstTraversal<S>
where
    S: State + OrdKey,
    <S as State>::HashKey: Eq,
    <S as OrdKey>::OrdKey: Ord,
{
    Traversal {
        queue: BinaryHeap::from([Reverse(StateWrapper(start))]),
        visited: HashSet::new(),
    }
}
