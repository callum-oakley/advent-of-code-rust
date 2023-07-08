use std::collections::{HashMap, HashSet};

use crate::search;

fn parse(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut res = HashMap::new();
    for line in input.lines() {
        let (prog, neighbors) = line.split_once(" <-> ").unwrap();
        res.insert(
            prog.trim().parse().unwrap(),
            neighbors.split(", ").map(|s| s.parse().unwrap()).collect(),
        );
    }
    res
}

#[derive(Clone)]
struct State<'a> {
    graph: &'a HashMap<u32, Vec<u32>>,
    pos: u32,
}

impl<'a> search::State for State<'a> {
    type HashKey = u32;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(self.graph[&self.pos].iter().map(|&pos| State {
            pos,
            graph: self.graph,
        }))
    }

    fn hash_key(&self) -> Self::HashKey {
        self.pos
    }
}

pub fn part1(input: &str) -> usize {
    search::depth_first(State {
        graph: &parse(input),
        pos: 0,
    })
    .count()
}

pub fn part2(input: &str) -> usize {
    let graph = parse(input);

    let mut components = 0;
    let mut unexplored: HashSet<_> = graph.keys().copied().collect();

    while !unexplored.is_empty() {
        components += 1;
        for State { pos, .. } in search::depth_first(State {
            pos: *unexplored.iter().next().unwrap(),
            graph: &graph,
        }) {
            unexplored.remove(&pos);
        }
    }

    components
}

pub fn tests() {
    let example = "0 <-> 2
                   1 <-> 1
                   2 <-> 0, 3, 4
                   3 <-> 2, 4
                   4 <-> 2, 3, 6
                   5 <-> 6
                   6 <-> 4, 5";
    assert_eq!(part1(example), 6);
    assert_eq!(part2(example), 2);
}
