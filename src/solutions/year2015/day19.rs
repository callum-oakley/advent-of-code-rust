use std::mem;

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

#[derive(Clone)]
struct State<'a> {
    molecule: String,
    steps: usize,
    reactions: &'a [(&'a str, &'a str)],
}

impl<'a, 'b> search::State for &'b State<'a> {
    type Adjacent = Vec<State<'a>>;
    type HashKey = &'b str;

    fn adjacent(self) -> Self::Adjacent {
        step(self.reactions, &self.molecule)
            .map(|molecule| State {
                molecule,
                steps: self.steps + 1,
                reactions: self.reactions,
            })
            // Would be nice to avoid this collect but the concrete type of the
            // iterator is a mess.
            .collect()
    }

    fn hash_key(self) -> Self::HashKey {
        &self.molecule
    }
}

impl<'a, 'b> search::OrdKey for &'b State<'a> {
    type OrdKey = usize;

    fn ord_key(self) -> Self::OrdKey {
        // The molecule length is NOT an admissible heuristic, but the
        // relaxation returns the correct answer in this case.
        self.steps + self.molecule.len()
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
