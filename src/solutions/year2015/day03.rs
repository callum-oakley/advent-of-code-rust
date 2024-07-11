use std::{collections::HashSet, iter::once};

use nalgebra::Vector2;

use crate::grid2::{IntoVector, Z};

fn deliver(dirs: impl Iterator<Item = char>) -> HashSet<Vector2<i32>> {
    let mut santa = Z;
    once(santa)
        .chain(dirs.map(|c| {
            santa += c.into_vector();
            santa
        }))
        .collect()
}

pub fn part1(input: &str) -> usize {
    deliver(input.chars()).len()
}

pub fn part2(input: &str) -> usize {
    deliver(input.chars().step_by(2))
        .union(&deliver(input.chars().skip(1).step_by(2)))
        .count()
}

pub fn tests() {
    assert_eq!(part1(">"), 2);
    assert_eq!(part1("^>v<"), 4);
    assert_eq!(part1("^v^v^v^v^v"), 2);

    assert_eq!(part2("^v"), 3);
    assert_eq!(part2("^>v<"), 3);
    assert_eq!(part2("^v^v^v^v^v"), 11);
}
