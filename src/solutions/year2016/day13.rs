use std::iter;

use crate::{
    grid::Point,
    search2::{self, Queue},
};

fn is_open(seed: u32, Point { x, y }: Point) -> bool {
    (u32::try_from(x * x + 3 * x + 2 * x * y + y + y * y).unwrap() + seed).count_ones() % 2 == 0
}

#[derive(Clone)]
struct State {
    pos: Point,
    steps: u32,
}

fn traversal(input: &str) -> impl Iterator<Item = State> {
    let seed = input.parse().unwrap();
    let mut q = search2::breadth_first(
        State {
            pos: Point { x: 1, y: 1 },
            steps: 0,
        },
        |state| state.pos,
    );
    iter::from_fn(move || {
        q.pop().map(|state| {
            for pos in state.pos.adjacent4() {
                if pos.x >= 0 && pos.y >= 0 && is_open(seed, pos) {
                    q.push(State {
                        pos,
                        steps: state.steps + 1,
                    });
                }
            }
            state
        })
    })
}

fn part1_(target: Point, input: &str) -> u32 {
    traversal(input)
        .find(|state| state.pos == target)
        .unwrap()
        .steps
}

pub fn part1(input: &str) -> u32 {
    part1_(Point { x: 31, y: 39 }, input)
}

pub fn part2(input: &str) -> usize {
    traversal(input)
        .take_while(|state| state.steps <= 50)
        .count()
}

pub fn tests() {
    assert_eq!(part1_(Point { x: 7, y: 4 }, "10"), 11);
}
