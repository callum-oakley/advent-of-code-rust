use std::collections::HashMap;

use crate::grid::{Point, Rect, Turn, N};

trait State {
    fn tick(&mut self, dir: &mut Point, infected_count: &mut usize);
}

#[derive(Clone, Default)]
enum State1 {
    #[default]
    Clean,
    Infected,
}

impl State for State1 {
    fn tick(&mut self, dir: &mut Point, infected_count: &mut usize) {
        match self {
            State1::Clean => {
                *dir = dir.turn(Turn::Left);
                *self = State1::Infected;
                *infected_count += 1;
            }
            State1::Infected => {
                *dir = dir.turn(Turn::Right);
                *self = State1::Clean;
            }
        }
    }
}

impl From<char> for State1 {
    fn from(c: char) -> Self {
        if c == '#' {
            State1::Infected
        } else {
            State1::Clean
        }
    }
}

#[derive(Clone, Default)]
enum State2 {
    #[default]
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State for State2 {
    fn tick(&mut self, dir: &mut Point, infected_count: &mut usize) {
        match self {
            State2::Clean => {
                *dir = dir.turn(Turn::Left);
                *self = State2::Weakened;
            }
            State2::Weakened => {
                *self = State2::Infected;
                *infected_count += 1;
            }
            State2::Infected => {
                *dir = dir.turn(Turn::Right);
                *self = State2::Flagged;
            }
            State2::Flagged => {
                *dir *= -1;
                *self = State2::Clean;
            }
        }
    }
}

impl From<char> for State2 {
    fn from(c: char) -> Self {
        if c == '#' {
            State2::Infected
        } else {
            State2::Clean
        }
    }
}

fn part_<T>(bursts: usize, input: &str) -> usize
where
    T: State + From<char> + Clone + Default,
{
    let nodes = Rect::parse(input, T::from);
    let mut pos = Point {
        x: nodes.size.x / 2,
        y: nodes.size.y / 2,
    };
    let mut dir = N;
    let mut nodes: HashMap<Point, T> = nodes.iter().map(|(p, s)| (p, s.clone())).collect();
    let mut infected_count = 0;
    for _ in 0..bursts {
        nodes
            .entry(pos)
            .or_default()
            .tick(&mut dir, &mut infected_count);
        pos += dir;
    }
    infected_count
}

pub fn part1(input: &str) -> usize {
    part_::<State1>(10_000, input)
}

pub fn part2(input: &str) -> usize {
    part_::<State2>(10_000_000, input)
}

pub fn tests() {
    assert_eq!(part1("..#\n#..\n..."), 5587);
    assert_eq!(part2("..#\n#..\n..."), 2_511_944);
}
