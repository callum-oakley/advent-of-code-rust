use std::collections::HashMap;

use crate::{
    grid::{Point, Rect, N, Z},
    intcode::{State, VM},
    ocr,
};

fn paint(input: &str, hull: &mut HashMap<Point, i64>) {
    let mut pos = Z;
    let mut dir = N;
    let mut vm = VM::new(input);
    while vm.state() != State::Halt {
        vm.input(hull.get(&pos).copied().unwrap_or_default());
        hull.insert(pos, vm.output());
        dir = dir.turn(vm.output().into());
        pos += dir;
    }
}

pub fn part1(input: &str) -> usize {
    let mut hull = HashMap::new();
    paint(input, &mut hull);
    hull.len()
}

pub fn part2(input: &str) -> &str {
    let mut hull = HashMap::from([(Z, 1)]);
    paint(input, &mut hull);
    ocr::parse(
        &Rect::from(
            hull.iter()
                .filter(|(_, &paint)| paint == 1)
                .map(|(&pos, _)| pos),
        )
        .to_string(),
    )
}
