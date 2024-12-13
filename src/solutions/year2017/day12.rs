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

fn search(graph: &HashMap<u32, Vec<u32>>, start: u32) -> impl Iterator<Item = u32> + '_ {
    search::breadth_first(
        start,
        |&pos| pos,
        |&pos, push| graph[&pos].iter().copied().for_each(push),
    )
}

pub fn part1(input: &str) -> usize {
    search(&parse(input), 0).count()
}

pub fn part2(input: &str) -> usize {
    let graph = parse(input);

    let mut components = 0;
    let mut unexplored: HashSet<_> = graph.keys().copied().collect();

    while !unexplored.is_empty() {
        components += 1;
        for pos in search(&graph, *unexplored.iter().next().unwrap()) {
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
