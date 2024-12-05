use std::{cmp::Ordering, collections::HashSet};

fn parse(input: &str) -> (HashSet<(u32, u32)>, impl Iterator<Item = Vec<u32>> + '_) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    (
        rules
            .trim()
            .lines()
            .map(|line| {
                let (a, b) = line.trim().split_once('|').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect(),
        updates.trim().lines().map(|line| {
            line.trim()
                .split(',')
                .map(|page| page.parse().unwrap())
                .collect()
        }),
    )
}

fn compare(rules: &HashSet<(u32, u32)>, a: u32, b: u32) -> Ordering {
    if rules.contains(&(a, b)) {
        Ordering::Less
    } else if rules.contains(&(b, a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn part1(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    updates
        .filter(|update| update.is_sorted_by(|&a, &b| compare(&rules, a, b).is_le()))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    updates
        .filter(|update| !update.is_sorted_by(|&a, &b| compare(&rules, a, b).is_le()))
        .map(|mut update| {
            update.sort_unstable_by(|&a, &b| compare(&rules, a, b));
            update[update.len() / 2]
        })
        .sum()
}

pub fn tests() {
    let example = "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";
    assert_eq!(part1(example), 143);
    assert_eq!(part2(example), 123);
}
