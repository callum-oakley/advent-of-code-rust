use std::collections::HashSet;

use crate::part::Part;

fn valid(part: Part, phrase: &str) -> bool {
    let mut seen = HashSet::new();
    for word in phrase.split_whitespace() {
        let mut word = Vec::from(word);
        if part == Part::Two {
            word.sort_unstable();
        }

        if seen.contains(&word) {
            return false;
        }
        seen.insert(word);
    }
    true
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|phrase| valid(Part::One, phrase))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|phrase| valid(Part::Two, phrase))
        .count()
}

pub fn tests() {
    assert!(valid(Part::One, "aa bb cc dd ee"));
    assert!(!valid(Part::One, "aa bb cc dd aa"));
    assert!(valid(Part::One, "aa bb cc dd aaa"));

    assert!(valid(Part::Two, "abcde fghij"));
    assert!(!valid(Part::Two, "abcde xyz ecdab"));
    assert!(valid(Part::Two, "a ab abc abd abf abj"));
    assert!(valid(Part::Two, "iiii oiii ooii oooi oooo"));
    assert!(!valid(Part::Two, "oiii ioii iioi iiio"));
}
