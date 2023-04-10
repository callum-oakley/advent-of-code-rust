use std::collections::HashMap;

use regex::Regex;

use crate::{combinatorics::Permutations, uniq::Uniq};

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

fn part_<'a>(distances: &'a HashMap<(&'a str, &'a str), u32>) -> impl Iterator<Item = u32> + 'a {
    Permutations::new(Uniq::new(distances.keys().map(|(a, _)| *a)).collect()).map(|route| {
        route
            .windows(2)
            .map(|pair| distances[&(pair[0], pair[1])])
            .sum()
    })
}

pub fn part1(input: &str) -> u32 {
    part_(&parse(input)).min().unwrap()
}

pub fn part2(input: &str) -> u32 {
    part_(&parse(input)).max().unwrap()
}

pub fn tests() {
    let example = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
    assert_eq!(part1(example), 605);
    assert_eq!(part2(example), 982);
}
