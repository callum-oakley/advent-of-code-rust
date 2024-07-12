use std::iter;

use crate::{
    grid2::{self, Vector},
    search::{self, Queue},
};

fn is_open(seed: u32, v: Vector) -> bool {
    (u32::try_from(v.x * v.x + 3 * v.x + 2 * v.x * v.y + v.y + v.y * v.y).unwrap() + seed)
        .count_ones()
        % 2
        == 0
}

#[derive(Clone)]
struct State {
    pos: Vector,
    steps: u32,
}

fn traversal(input: &str) -> impl Iterator<Item = State> {
    let seed = input.parse().unwrap();
    let mut q = search::breadth_first(
        State {
            pos: Vector::new(1, 1),
            steps: 0,
        },
        |state| state.pos,
    );
    iter::from_fn(move || {
        q.pop().map(|state| {
            for pos in grid2::adjacent4(state.pos) {
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

fn part1_(target: Vector, input: &str) -> u32 {
    traversal(input)
        .find(|state| state.pos == target)
        .unwrap()
        .steps
}

pub fn part1(input: &str) -> u32 {
    part1_(Vector::new(31, 39), input)
}

pub fn part2(input: &str) -> usize {
    traversal(input)
        .take_while(|state| state.steps <= 50)
        .count()
}

pub fn tests() {
    assert_eq!(part1_(Vector::new(7, 4), "10"), 11);
}
