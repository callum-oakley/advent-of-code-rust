use std::cmp;

use crate::grid_hex::{self, Point, Z};

pub fn part1(input: &str) -> i32 {
    grid_hex::dist(input.split(',').map(grid_hex::from_str).sum::<Point>())
}

pub fn part2(input: &str) -> i32 {
    let mut furthest = 0;
    let mut pos = Z;
    for step in input.split(',').map(grid_hex::from_str) {
        pos += step;
        furthest = cmp::max(furthest, grid_hex::dist(pos));
    }
    furthest
}

pub fn tests() {
    assert_eq!(part1("ne,ne,ne"), 3);
    assert_eq!(part1("ne,ne,sw,sw"), 0);
    assert_eq!(part1("ne,ne,s,s"), 2);
    assert_eq!(part1("se,sw,se,sw,sw"), 3);
}
