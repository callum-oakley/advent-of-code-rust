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
    step(&reactions, molecule).uniq().count()
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct State {
    molecule: String,
    steps: usize,
}

pub fn part2(input: &str) -> usize {
    let (mut reactions, molecule) = parse(input);
    for (a, b) in &mut reactions {
        mem::swap(a, b);
    }

    search::a_star(
        State {
            molecule: molecule.to_owned(),
            steps: 0,
        },
        move |state, push| {
            for molecule in step(&reactions, &state.molecule) {
                push(State {
                    molecule,
                    steps: state.steps + 1,
                });
            }
        },
        search::hash_filter(|state: &State| state.molecule.clone()),
        |state| state.steps,
        // The molecule length is NOT an admissible heuristic, but the relaxation returns the
        // correct answer in this case.
        |state| state.molecule.len(),
    )
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
        6,
    );
}
