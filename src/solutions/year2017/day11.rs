use std::cmp;

use crate::hex_grid::{Axial, Z};

pub fn part1(input: &str) -> i32 {
    input.split(',').map(Axial::from).sum::<Axial>().dist()
}

pub fn part2(input: &str) -> i32 {
    let mut furthest = 0;
    let mut pos = Z;
    for step in input.split(',').map(Axial::from) {
        pos += step;
        furthest = cmp::max(furthest, pos.dist());
    }
    furthest
}

pub fn tests() {
    assert_eq!(part1("ne,ne,ne"), 3);
    assert_eq!(part1("ne,ne,sw,sw"), 0);
    assert_eq!(part1("ne,ne,s,s"), 2);
    assert_eq!(part1("se,sw,se,sw,sw"), 3);
}
