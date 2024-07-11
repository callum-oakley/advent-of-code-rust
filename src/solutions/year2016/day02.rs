use std::collections::HashMap;

use nalgebra::Vector2;

use crate::grid2::{IntoVector, E, N, NE, NW, S, SE, SW, W, Z};

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = Vector2<i32>> + '_> + '_ {
    input
        .split_whitespace()
        .map(|s| s.chars().map(IntoVector::into_vector))
}

fn part_(keypad: &HashMap<Vector2<i32>, char>, input: &str) -> String {
    let mut res = String::new();
    let mut pos = Z;
    for instruction in parse(input) {
        for dir in instruction {
            if keypad.contains_key(&(pos + dir)) {
                pos += dir;
            }
        }
        res.push(keypad[&pos]);
    }
    res
}

pub fn part1(input: &str) -> String {
    part_(
        &[
            (NW, '1'),
            (N, '2'),
            (NE, '3'),
            (W, '4'),
            (Z, '5'),
            (E, '6'),
            (SW, '7'),
            (S, '8'),
            (SE, '9'),
        ]
        .into(),
        input,
    )
}

pub fn part2(input: &str) -> String {
    part_(
        &[
            (Vector2::new(2, -2), '1'),
            (Vector2::new(1, -1), '2'),
            (Vector2::new(2, -1), '3'),
            (Vector2::new(3, -1), '4'),
            (Vector2::new(0, 0), '5'),
            (Vector2::new(1, 0), '6'),
            (Vector2::new(2, 0), '7'),
            (Vector2::new(3, 0), '8'),
            (Vector2::new(4, 0), '9'),
            (Vector2::new(1, 1), 'A'),
            (Vector2::new(2, 1), 'B'),
            (Vector2::new(3, 1), 'C'),
            (Vector2::new(2, 2), 'D'),
        ]
        .into(),
        input,
    )
}

pub fn tests() {
    assert_eq!(part1("ULL RRDDD LURDL UUUUD"), "1985");
    assert_eq!(part2("ULL RRDDD LURDL UUUUD"), "5DB3");
}
