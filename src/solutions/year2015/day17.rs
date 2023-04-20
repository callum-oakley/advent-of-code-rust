use std::{
    cmp::Ordering,
    iter::{empty, once},
};

use lazy_static::lazy_static;
use regex::Regex;

fn parse(input: &str) -> Vec<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let mut containers: Vec<_> = RE
        .find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    containers.sort_unstable();
    containers
}

fn combinations(eggnog: u32, containers: &[u32]) -> Box<dyn Iterator<Item = Vec<u32>> + '_> {
    if eggnog == 0 {
        Box::new(once(vec![]))
    } else if containers.is_empty() || containers[0] > eggnog {
        Box::new(empty())
    } else {
        Box::new(
            combinations(eggnog - containers[0], &containers[1..])
                .map(move |mut combination| {
                    combination.push(containers[0]);
                    combination
                })
                .chain(combinations(eggnog, &containers[1..])),
        )
    }
}

fn _part1(eggnog: u32, input: &str) -> usize {
    combinations(eggnog, &parse(input)).count()
}

fn _part2(eggnog: u32, input: &str) -> usize {
    let containers = parse(input);
    let mut lengths = combinations(eggnog, &containers).map(|combination| combination.len());
    let mut min_len = lengths.next().unwrap();
    let mut count = 1;
    for len in lengths {
        match len.cmp(&min_len) {
            Ordering::Equal => {
                count += 1;
            }
            Ordering::Less => {
                min_len = len;
                count = 1;
            }
            Ordering::Greater => {}
        }
    }
    count
}

pub fn part1(input: &str) -> usize {
    _part1(150, input)
}

pub fn part2(input: &str) -> usize {
    _part2(150, input)
}

pub fn tests() {
    assert_eq!(_part1(25, "20 15 10 5 5"), 4);
    assert_eq!(_part2(25, "20 15 10 5 5"), 3);
}
