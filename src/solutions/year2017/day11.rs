use std::cmp;

use crate::hex_grid::{self, Point, Z};

pub fn part1(input: &str) -> i32 {
    hex_grid::dist(input.split(',').map(hex_grid::from_str).sum::<Point>())
}

pub fn part2(input: &str) -> i32 {
    let mut furthest = 0;
    let mut pos = Z;
    for step in input.split(',').map(hex_grid::from_str) {
        pos += step;
        furthest = cmp::max(furthest, hex_grid::dist(pos));
    }
    furthest
}

pub fn tests() {
    assert_eq!(part1("ne,ne,ne"), 3);
    assert_eq!(part1("ne,ne,sw,sw"), 0);
    assert_eq!(part1("ne,ne,s,s"), 2);
    assert_eq!(part1("se,sw,se,sw,sw"), 3);
}
