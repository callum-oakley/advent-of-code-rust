use std::{
    collections::HashMap,
    ops::{Div, Mul, Rem},
};

use crate::grid2::{Grid, Vector, Z};

fn rotate(pattern: &Grid<bool>) -> Grid<bool> {
    let mut res = pattern.clone();
    for (v, &pixel) in pattern {
        res[[pattern.size.x - 1 - v.y, v.x]] = pixel;
    }
    res
}

fn reflect(pattern: &Grid<bool>) -> Grid<bool> {
    let mut res = pattern.clone();
    for (v, &pixel) in pattern {
        res[[pattern.size.x - 1 - v.x, v.y]] = pixel;
    }
    res
}

fn symmetries(pattern: &Grid<bool>) -> [Grid<bool>; 8] {
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

fn parse_pattern(s: &str) -> Grid<bool> {
    Grid::parse(&s.replace('/', "\n"), |_, c| c == '#')
}

fn parse_rules(input: &str) -> HashMap<Grid<bool>, Grid<bool>> {
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

fn partition(pattern: &Grid<bool>) -> Grid<Grid<bool>> {
    let inner_size = if pattern.size.x % 2 == 0 {
        Vector::new(2, 2)
    } else {
        Vector::new(3, 3)
    };

    let mut res = Grid::new(
        Grid::new(false, inner_size),
        pattern.size.zip_map(&inner_size, Div::div),
    );

    for (pos, &pixel) in pattern {
        res[pos.zip_map(&inner_size, Div::div)][pos.zip_map(&inner_size, Rem::rem)] = pixel;
    }

    res
}

fn enhance(rules: &HashMap<Grid<bool>, Grid<bool>>, blocks: &Grid<Grid<bool>>) -> Grid<Grid<bool>> {
    let mut res = blocks.clone();
    for pos in blocks.keys() {
        res[pos] = rules[&blocks[pos]].clone();
    }
    res
}

fn collapse(blocks: &Grid<Grid<bool>>) -> Grid<bool> {
    let inner_size = blocks[Z].size;
    let mut res = Grid::new(false, inner_size.zip_map(&blocks.size, Mul::mul));

    for (outer_pos, block) in blocks {
        for (inner_pos, &pixel) in block {
            res[outer_pos.zip_map(&inner_size, Mul::mul) + inner_pos] = pixel;
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
