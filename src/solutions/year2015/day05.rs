use std::sync::LazyLock;

use regex::Regex;

fn is_nice1(s: &str) -> bool {
    static VOWELS: LazyLock<Regex> = LazyLock::new(|| Regex::new("a|e|i|o|u").unwrap());
    static AB_CD_PQ_XY: LazyLock<Regex> = LazyLock::new(|| Regex::new("ab|cd|pq|xy").unwrap());
    VOWELS.find_iter(s).count() >= 3
        && s.as_bytes().windows(2).any(|pair| pair[0] == pair[1])
        && !AB_CD_PQ_XY.is_match(s)
}

fn has_pair_of_pairs(s: &str) -> bool {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() - 1 {
        for j in i + 2..bytes.len() - 1 {
            if bytes[i..i + 2] == bytes[j..j + 2] {
                return true;
            }
        }
    }
    false
}

fn is_nice2(s: &str) -> bool {
    has_pair_of_pairs(s) && s.as_bytes().windows(3).any(|triple| triple[0] == triple[2])
}

pub fn part1(input: &str) -> usize {
    input.lines().filter(|s| is_nice1(s)).count()
}

pub fn part2(input: &str) -> usize {
    input.lines().filter(|s| is_nice2(s)).count()
}

pub fn tests() {
    assert!(is_nice1("ugknbfddgicrmopn"));
    assert!(is_nice1("aaa"));
    assert!(!is_nice1("jchzalrnumimnmhp"));
    assert!(!is_nice1("haegwjzuvuyypxyu"));
    assert!(!is_nice1("dvszwmarrgswjxmb"));

    assert!(is_nice2("qjhvhtzxzqqjkmpb"));
    assert!(is_nice2("xxyxx"));
    assert!(!is_nice2("uurcxstgmygtbstg"));
    assert!(!is_nice2("ieodomkazucvgmuy"));
}
