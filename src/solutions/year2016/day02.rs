use std::collections::HashMap;

use crate::grid::{Point, E, N, NE, NW, S, SE, SW, W, Z};

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = Point> + '_> + '_ {
    input.split_whitespace().map(|s| s.chars().map(Point::from))
}

fn part_(keypad: &HashMap<Point, char>, input: &str) -> String {
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
            (Point { x: 2, y: -2 }, '1'),
            (Point { x: 1, y: -1 }, '2'),
            (Point { x: 2, y: -1 }, '3'),
            (Point { x: 3, y: -1 }, '4'),
            (Point { x: 0, y: 0 }, '5'),
            (Point { x: 1, y: 0 }, '6'),
            (Point { x: 2, y: 0 }, '7'),
            (Point { x: 3, y: 0 }, '8'),
            (Point { x: 4, y: 0 }, '9'),
            (Point { x: 1, y: 1 }, 'A'),
            (Point { x: 2, y: 1 }, 'B'),
            (Point { x: 3, y: 1 }, 'C'),
            (Point { x: 2, y: 2 }, 'D'),
        ]
        .into(),
        input,
    )
}

pub fn tests() {
    assert_eq!(part1("ULL RRDDD LURDL UUUUD"), "1985");
    assert_eq!(part2("ULL RRDDD LURDL UUUUD"), "5DB3");
}
