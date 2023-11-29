use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::{
    combinatorics::permute,
    grid::{Point, Rect},
    search2::{self, Queue},
};

fn distance(ducts: &HashSet<Point>, a: Point, b: Point) -> u32 {
    struct State {
        pos: Point,
        steps: u32,
    }
    let mut q = search2::breadth_first(State { pos: a, steps: 0 }, |state| state.pos);
    while let Some(state) = q.pop() {
        if state.pos == b {
            return state.steps;
        }
        for pos in state.pos.adjacent4() {
            if ducts.contains(&pos) {
                q.push(State {
                    pos,
                    steps: state.steps + 1,
                });
            }
        }
    }
    unreachable!()
}

fn total_dist(dists: &HashMap<(char, char), u32>, route: &[char]) -> u32 {
    route
        .windows(2)
        .map(|pair| dists[&(pair[0], pair[1])])
        .sum()
}

fn part_(return_to_start: bool, input: &str) -> u32 {
    let map = Rect::parse(input, |_, c| c);

    let nodes: Vec<(Point, char)> = map
        .iter()
        .filter(|(_, c)| c.is_ascii_digit())
        .map(|(p, c)| (p, *c))
        .collect();

    let ducts: HashSet<Point> = map
        .iter()
        .filter(|(_, c)| c.is_ascii_digit() || **c == '.')
        .map(|(p, _)| p)
        .collect();

    let mut dists: HashMap<(char, char), u32> = HashMap::new();
    for (i, &(a_p, a)) in nodes.iter().enumerate() {
        for &(b_p, b) in &nodes[i + 1..] {
            let dist = distance(&ducts, a_p, b_p);
            dists.insert((a, b), dist);
            dists.insert((b, a), dist);
        }
    }

    let nodes_len = nodes.len();

    let mut route: Vec<char> = nodes.into_iter().map(|(_, c)| c).collect();
    route.sort_unstable();
    if return_to_start {
        route.push('0');
    }

    let mut best_total_dist = total_dist(&dists, &route);
    while permute(&mut route[1..nodes_len]) {
        best_total_dist = cmp::min(best_total_dist, total_dist(&dists, &route));
    }

    best_total_dist
}

pub fn part1(input: &str) -> u32 {
    part_(false, input)
}

pub fn part2(input: &str) -> u32 {
    part_(true, input)
}

pub fn tests() {
    let example = "###########\n#0.1.....2#\n#.#######.#\n#4.......3#\n###########";
    assert_eq!(part_(false, example), 14);
}
