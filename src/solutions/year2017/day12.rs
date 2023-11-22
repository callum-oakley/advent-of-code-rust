use std::collections::{HashMap, HashSet};

use crate::search2::{self, Queue};

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

pub fn part1(input: &str) -> usize {
    let graph = parse(input);
    let mut q = search2::depth_first(0, |&pos| pos);
    let mut count = 0;
    while let Some(pos) = q.pop() {
        count += 1;
        for &p in &graph[&pos] {
            q.push(p);
        }
    }
    count
}

pub fn part2(input: &str) -> usize {
    let graph = parse(input);

    let mut components = 0;
    let mut unexplored: HashSet<_> = graph.keys().copied().collect();

    while !unexplored.is_empty() {
        components += 1;
        let mut q = search2::depth_first(*unexplored.iter().next().unwrap(), |&pos| pos);
        while let Some(pos) = q.pop() {
            unexplored.remove(&pos);
            for &p in &graph[&pos] {
                q.push(p);
            }
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
