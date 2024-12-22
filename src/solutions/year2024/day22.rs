use std::{collections::HashMap, iter};

fn parse(input: &str) -> impl Iterator<Item = i64> + '_ {
    input.split_whitespace().map(|s| s.parse().unwrap())
}

fn secrets(seed: i64) -> impl Iterator<Item = i64> {
    iter::successors(Some(seed), |&(mut secret)| {
        secret = (secret ^ (secret * 64)) % 16_777_216;
        secret = (secret ^ (secret / 32)) % 16_777_216;
        secret = (secret ^ (secret * 2048)) % 16_777_216;
        Some(secret)
    })
}

pub fn part1(input: &str) -> i64 {
    parse(input)
        .map(|seed| secrets(seed).nth(2000).unwrap())
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let monkeys_to_prices: Vec<Vec<i64>> = parse(input)
        .map(|seed| secrets(seed).map(|secret| secret % 10).take(2001).collect())
        .collect();

    let monkeys_to_changes_to_bananas: Vec<HashMap<Vec<i64>, i64>> = monkeys_to_prices
        .iter()
        .map(|p| {
            (0..p.len() - 4)
                .rev()
                .map(|i| ((0..4).map(|j| p[i + j + 1] - p[i + j]).collect(), p[i + 4]))
                .collect()
        })
        .collect();

    let mut changes_to_total_bananas: HashMap<Vec<i64>, i64> = HashMap::new();
    for changes_to_bananas in monkeys_to_changes_to_bananas {
        for (changes, bananas) in changes_to_bananas {
            *changes_to_total_bananas.entry(changes).or_default() += bananas;
        }
    }

    *changes_to_total_bananas.values().max().unwrap()
}

pub fn tests() {
    assert_eq!(part1("1 10 100 2024"), 37_327_623);
    assert_eq!(part2("1 2 3 2024"), 23);
}
