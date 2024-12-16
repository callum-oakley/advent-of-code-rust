use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet, VecDeque},
    hash::Hash,
    iter,
    ops::Add,
};

trait Queue {
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

struct CostHeap<V, C, O> {
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

fn search<Q, S, H, K, A>(
    mut queue: Q,
    start: S,
    mut hash_key: H,
    mut adjacent: A,
) -> impl Iterator<Item = S>
where
    Q: Queue<Item = S>,
    H: FnMut(&S) -> K,
    K: Eq + Hash,
    A: FnMut(&S, &mut dyn FnMut(S)),
{
    let mut visited = HashSet::new();
    queue.push(start);
    iter::from_fn(move || {
        while let Some(state) = queue.pop() {
            let key = hash_key(&state);
            if !visited.contains(&key) {
                visited.insert(key);
                adjacent(&state, &mut |a| queue.push(a));
                return Some(state);
            }
        }
        None
    })
}

/// Search a state space breadth first.
pub fn breadth_first<S, H, K, A>(start: S, hash_key: H, adjacent: A) -> impl Iterator<Item = S>
where
    H: FnMut(&S) -> K,
    K: Eq + Hash,
    A: FnMut(&S, &mut dyn FnMut(S)),
{
    search(VecDeque::new(), start, hash_key, adjacent)
}

/// Search a state space min-cost first.
pub fn dijkstra<S, H, K, A, C, O>(
    start: S,
    hash_key: H,
    cost: C,
    adjacent: A,
) -> impl Iterator<Item = S>
where
    H: FnMut(&S) -> K,
    K: Eq + Hash,
    A: FnMut(&S, &mut dyn FnMut(S)),
    C: FnMut(&S) -> O,
    O: Ord,
{
    search(
        CostHeap {
            cost,
            binary_heap: BinaryHeap::new(),
        },
        start,
        hash_key,
        adjacent,
    )
}

/// Search a state space min-cost-plus-heuristic first.
pub fn a_star<S, H, K, A, C, D, O>(
    start: S,
    hash_key: H,
    mut cost: C,
    mut heuristic: D,
    adjacent: A,
) -> impl Iterator<Item = S>
where
    H: FnMut(&S) -> K,
    K: Eq + Hash,
    A: FnMut(&S, &mut dyn FnMut(S)),
    C: FnMut(&S) -> O,
    D: FnMut(&S) -> O,
    O: Add,
    O::Output: Ord,
{
    dijkstra(
        start,
        hash_key,
        move |state| cost(state) + heuristic(state),
        adjacent,
    )
}

fn search_nohash<Q, S, A>(mut queue: Q, start: S, mut adjacent: A) -> impl Iterator<Item = S>
where
    Q: Queue<Item = S>,
    A: FnMut(&S, &mut dyn FnMut(S)),
{
    queue.push(start);
    iter::from_fn(move || {
        queue
            .pop()
            .inspect(|state| adjacent(state, &mut |a| queue.push(a)))
    })
}

/// Search a state space breadth first (may visit the same state multiple times).
pub fn breadth_first_nohash<S, A>(start: S, adjacent: A) -> impl Iterator<Item = S>
where
    A: FnMut(&S, &mut dyn FnMut(S)),
{
    search_nohash(VecDeque::new(), start, adjacent)
}

/// Search a state space min-cost first (may visit the same state multiple times).
pub fn dijkstra_nohash<S, A, C, O>(start: S, cost: C, adjacent: A) -> impl Iterator<Item = S>
where
    A: FnMut(&S, &mut dyn FnMut(S)),
    C: FnMut(&S) -> O,
    O: Ord,
{
    search_nohash(
        CostHeap {
            cost,
            binary_heap: BinaryHeap::new(),
        },
        start,
        adjacent,
    )
}
