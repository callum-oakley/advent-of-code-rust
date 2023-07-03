use std::collections::HashSet;

fn valid(part: u8, phrase: &str) -> bool {
    let mut seen = HashSet::new();
    for word in phrase.split_whitespace() {
        let mut word = Vec::from(word);
        if part == 2 {
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
    input.lines().filter(|phrase| valid(1, phrase)).count()
}

pub fn part2(input: &str) -> usize {
    input.lines().filter(|phrase| valid(2, phrase)).count()
}

pub fn tests() {
    assert!(valid(1, "aa bb cc dd ee"));
    assert!(!valid(1, "aa bb cc dd aa"));
    assert!(valid(1, "aa bb cc dd aaa"));

    assert!(valid(2, "abcde fghij"));
    assert!(!valid(2, "abcde xyz ecdab"));
    assert!(valid(2, "a ab abc abd abf abj"));
    assert!(valid(2, "iiii oiii ooii oooi oooo"));
    assert!(!valid(2, "oiii ioii iioi iiio"));
}
