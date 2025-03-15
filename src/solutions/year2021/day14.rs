use std::collections::HashMap;

use regex::Regex;

use crate::freqs::Freqs;

// Represent a polymer by tracking the frequency of its elements and the frequency of each pair of
// elements.
struct Polymer {
    elements: HashMap<char, usize>,
    pairs: HashMap<(char, char), usize>,
}

fn parse(input: &str) -> (Polymer, HashMap<(char, char), char>) {
    let polymer = Regex::new(r"\w+").unwrap().find(input).unwrap().as_str();
    let elements = polymer.chars().freqs();
    let pairs = polymer
        .as_bytes()
        .windows(2)
        .map(|pair| (char::from(pair[0]), char::from(pair[1])))
        .freqs();
    let rules = Regex::new(r"(\w\w) -> (\w)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| {
            let pair = captures[1].as_bytes();
            (
                (char::from(pair[0]), char::from(pair[1])),
                char::from(captures[2].as_bytes()[0]),
            )
        })
        .collect();
    (Polymer { elements, pairs }, rules)
}

fn step(rules: &HashMap<(char, char), char>, polymer: &Polymer) -> Polymer {
    let mut elements = polymer.elements.clone();
    let mut pairs = HashMap::new();
    for (&(a, b), &n) in &polymer.pairs {
        let c = rules[&(a, b)];
        *elements.entry(c).or_default() += n;
        *pairs.entry((a, c)).or_default() += n;
        *pairs.entry((c, b)).or_default() += n;
    }
    Polymer { elements, pairs }
}

fn part_(steps: usize, input: &str) -> usize {
    let (mut polymer, rules) = parse(input);
    for _ in 0..steps {
        polymer = step(&rules, &polymer);
    }
    let values: Vec<_> = polymer.elements.values().copied().collect();
    values.iter().max().unwrap() - values.iter().min().unwrap()
}

pub fn part1(input: &str) -> usize {
    part_(10, input)
}

pub fn part2(input: &str) -> usize {
    part_(40, input)
}

pub fn tests() {
    let example = "
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    ";
    assert_eq!(part1(example), 1588);
    assert_eq!(part2(example), 2_188_189_693_529);
}
