use std::collections::HashMap;

use crate::grid::{Point, Rect, Z};

fn rotate(pattern: &Rect<bool>) -> Rect<bool> {
    let mut res = pattern.clone();
    for (Point { x, y }, &pixel) in pattern {
        res[Point {
            x: pattern.size.x - 1 - y,
            y: x,
        }] = pixel;
    }
    res
}

fn reflect(pattern: &Rect<bool>) -> Rect<bool> {
    let mut res = pattern.clone();
    for (Point { x, y }, &pixel) in pattern {
        res[Point {
            x: pattern.size.x - 1 - x,
            y,
        }] = pixel;
    }
    res
}

fn symmetries(pattern: &Rect<bool>) -> [Rect<bool>; 8] {
    [
        pattern.clone(),
        rotate(pattern),
        rotate(&rotate(pattern)),
        rotate(&rotate(&rotate(pattern))),
        reflect(pattern),
        rotate(&reflect(pattern)),
        rotate(&rotate(&reflect(pattern))),
        rotate(&rotate(&rotate(&reflect(pattern)))),
    ]
}

fn parse_pattern(s: &str) -> Rect<bool> {
    Rect::parse(&s.replace('/', "\n"), |c| c == '#')
}

fn parse_rules(input: &str) -> HashMap<Rect<bool>, Rect<bool>> {
    let mut res = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(" => ").unwrap();
        let from = parse_pattern(from);
        let to = parse_pattern(to);
        for from in symmetries(&from) {
            res.insert(from, to.clone());
        }
    }
    res
}

fn partition(pattern: &Rect<bool>) -> Rect<Rect<bool>> {
    let inner_size = if pattern.size.x % 2 == 0 {
        Point { x: 2, y: 2 }
    } else {
        Point { x: 3, y: 3 }
    };

    let mut res = Rect::new(Rect::new(false, inner_size), pattern.size / inner_size);

    for (pos, &pixel) in pattern {
        res[pos / inner_size][pos % inner_size] = pixel;
    }

    res
}

fn enhance(rules: &HashMap<Rect<bool>, Rect<bool>>, blocks: &Rect<Rect<bool>>) -> Rect<Rect<bool>> {
    let mut res = blocks.clone();
    for pos in blocks.keys() {
        res[pos] = rules[&blocks[pos]].clone();
    }
    res
}

fn collapse(blocks: &Rect<Rect<bool>>) -> Rect<bool> {
    let inner_size = blocks[Z].size;
    let mut res = Rect::new(false, inner_size * blocks.size);

    for (outer_pos, block) in blocks {
        for (inner_pos, &pixel) in block {
            res[outer_pos * inner_size + inner_pos] = pixel;
        }
    }

    res
}

fn part_(iterations: usize, input: &str) -> usize {
    let rules = parse_rules(input);
    let mut pattern = parse_pattern(".#./..#/###");

    for _ in 0..iterations {
        pattern = collapse(&enhance(&rules, &partition(&pattern)));
    }

    pattern.values().copied().filter(|p| *p).count()
}

pub fn part1(input: &str) -> usize {
    part_(5, input)
}

pub fn part2(input: &str) -> usize {
    part_(18, input)
}

pub fn tests() {
    let example = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";
    assert_eq!(part_(2, example), 12);
}
