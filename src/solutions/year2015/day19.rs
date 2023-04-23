use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    mem,
};

use crate::{search, uniq::Uniq};

fn parse(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (reactions, molecule) = input.split_once("\n\n").unwrap();
    (
        reactions
            .lines()
            .map(|line| line.split_once(" => ").unwrap())
            .collect(),
        molecule,
    )
}

fn step<'a>(
    reactions: &'a [(&'a str, &'a str)],
    molecule: &'a str,
) -> impl Iterator<Item = String> + 'a {
    reactions.iter().flat_map(|(reactant, product)| {
        molecule.match_indices(reactant).map(|(i, _)| {
            let mut res = String::new();
            res.push_str(&molecule[0..i]);
            res.push_str(product);
            res.push_str(&molecule[i + reactant.len()..]);
            res
        })
    })
}

pub fn part1(input: &str) -> usize {
    let (reactions, molecule) = parse(input);
    Uniq::new(step(&reactions, molecule)).count()
}

#[derive(Eq, Clone)]
struct State<'a> {
    molecule: String,
    steps: usize,
    reactions: &'a [(&'a str, &'a str)],
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.molecule == other.molecule
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.steps + self.molecule.len()).cmp(&(other.steps + other.molecule.len()))
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.molecule.hash(state);
    }
}

impl<'a> search::State for State<'a> {
    fn adjacent(&self) -> Vec<State<'a>> {
        step(self.reactions, &self.molecule)
            .map(|molecule| State {
                molecule,
                steps: self.steps + 1,
                reactions: self.reactions,
            })
            .collect()
    }
}

pub fn part2(input: &str) -> usize {
    let (mut reactions, molecule) = parse(input);
    for (a, b) in &mut reactions {
        mem::swap(a, b);
    }
    search::min_first(State {
        molecule: molecule.to_owned(),
        steps: 0,
        reactions: &reactions,
    })
    .find(|state| state.molecule == "e")
    .unwrap()
    .steps
}

pub fn tests() {
    assert_eq!(part1("H => HO\nH => OH\nO => HH\n\nHOH"), 4);
    assert_eq!(part1("H => HO\nH => OH\nO => HH\n\nHOHOHO"), 7);
    assert_eq!(part2("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH"), 3);
    assert_eq!(
        part2("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO"),
        6
    );
}
