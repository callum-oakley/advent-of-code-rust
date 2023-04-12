use std::{cmp::max, collections::HashMap};

use regex::Regex;

use crate::{combinatorics::permute, uniq::Uniq};

fn parse(input: &str) -> HashMap<(&str, &str), i32> {
    let mut preferences = HashMap::new();
    for captures in
        Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.")
            .unwrap()
            .captures_iter(input)
    {
        let a = captures.get(1).unwrap().as_str();
        let sign = if &captures[2] == "gain" { 1 } else { -1 };
        let happiness = sign * captures[3].parse::<i32>().unwrap();
        let b = captures.get(4).unwrap().as_str();
        *preferences.entry((a, b)).or_insert(0) += happiness;
        *preferences.entry((b, a)).or_insert(0) += happiness;
    }
    preferences
}

fn part_(preferences: &HashMap<(&str, &str), i32>) -> i32 {
    let happiness = |arrangement: &[&str]| {
        arrangement
            .windows(2)
            .map(|pair| preferences[&(pair[0], pair[1])])
            .sum::<i32>()
            + preferences[&(arrangement[arrangement.len() - 1], arrangement[0])]
    };
    let mut arrangement: Vec<_> = Uniq::new(preferences.keys().map(|(a, _)| *a)).collect();
    arrangement.sort_unstable();
    let mut best_happiness = happiness(&arrangement);
    // Since the arrangement is circular, we can fix the first element
    while permute(&mut arrangement[1..]) {
        best_happiness = max(best_happiness, happiness(&arrangement));
    }
    best_happiness
}

pub fn part1(input: &str) -> i32 {
    part_(&parse(input))
}

pub fn part2(input: &str) -> i32 {
    let mut preferences = parse(input);
    for guest in Uniq::new(preferences.keys().map(|(a, _)| *a)).collect::<Vec<_>>() {
        preferences.insert((guest, "me"), 0);
        preferences.insert(("me", guest), 0);
    }
    part_(&preferences)
}

pub fn tests() {
    assert_eq!(
        part1(
            "Alice would gain 54 happiness units by sitting next to Bob.
             Alice would lose 79 happiness units by sitting next to Carol.
             Alice would lose 2 happiness units by sitting next to David.
             Bob would gain 83 happiness units by sitting next to Alice.
             Bob would lose 7 happiness units by sitting next to Carol.
             Bob would lose 63 happiness units by sitting next to David.
             Carol would lose 62 happiness units by sitting next to Alice.
             Carol would gain 60 happiness units by sitting next to Bob.
             Carol would gain 55 happiness units by sitting next to David.
             David would gain 46 happiness units by sitting next to Alice.
             David would lose 7 happiness units by sitting next to Bob.
             David would gain 41 happiness units by sitting next to Carol."
        ),
        330
    );
}
