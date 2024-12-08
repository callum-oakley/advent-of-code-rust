use std::collections::HashMap;

use regex::Regex;

use crate::{combinatorics::permute, uniq::Uniq};

fn parse(input: &str) -> HashMap<(&str, &str), u32> {
    let mut distances = HashMap::new();
    for captures in Regex::new(r"(\w+) to (\w+) = (\d+)")
        .unwrap()
        .captures_iter(input)
    {
        let a = captures.get(1).unwrap().as_str();
        let b = captures.get(2).unwrap().as_str();
        let dist = captures[3].parse().unwrap();
        distances.insert((a, b), dist);
        distances.insert((b, a), dist);
    }
    distances
}

fn part_(input: &str, best: fn(u32, u32) -> u32) -> u32 {
    let distances = parse(input);
    let distance = |route: &[&str]| {
        route
            .windows(2)
            .map(|pair| distances[&(pair[0], pair[1])])
            .sum()
    };
    let mut route: Vec<_> = distances.keys().map(|(a, _)| *a).uniq().collect();
    route.sort_unstable();
    let mut best_distance = distance(&route);
    while permute(&mut route) {
        best_distance = best(best_distance, distance(&route));
    }
    best_distance
}

pub fn part1(input: &str) -> u32 {
    part_(input, std::cmp::min)
}

pub fn part2(input: &str) -> u32 {
    part_(input, std::cmp::max)
}

pub fn tests() {
    let example = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
    assert_eq!(part1(example), 605);
    assert_eq!(part2(example), 982);
}
