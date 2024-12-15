use std::{
    cmp::Ordering,
    iter::{empty, once},
    sync::LazyLock,
};

use regex::Regex;

fn parse(input: &str) -> Vec<u32> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
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

fn part1_(eggnog: u32, input: &str) -> usize {
    combinations(eggnog, &parse(input)).count()
}

fn part2_(eggnog: u32, input: &str) -> usize {
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
    part1_(150, input)
}

pub fn part2(input: &str) -> usize {
    part2_(150, input)
}

pub fn tests() {
    assert_eq!(part1_(25, "20 15 10 5 5"), 4);
    assert_eq!(part2_(25, "20 15 10 5 5"), 3);
}
