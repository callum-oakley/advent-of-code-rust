use std::{cell::RefCell, collections::HashMap};

use crate::{
    grid::{Point, N, W, Z},
    search2::{self, Queue},
};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

const TOOLS: [Tool; 3] = [Tool::Torch, Tool::ClimbingGear, Tool::Neither];

fn compatible(tile: Tile, tool: Tool) -> bool {
    match tile {
        Tile::Rocky => tool != Tool::Neither,
        Tile::Wet => tool != Tool::Torch,
        Tile::Narrow => tool != Tool::ClimbingGear,
    }
}

impl From<i32> for Tile {
    fn from(erosion: i32) -> Self {
        match erosion % 3 {
            0 => Tile::Rocky,
            1 => Tile::Wet,
            2 => Tile::Narrow,
            _ => unreachable!(),
        }
    }
}

struct Cave {
    depth: i32,
    target: Point,
    erosion_cache: RefCell<HashMap<Point, i32>>,
}

impl Cave {
    fn erosion(&self, pos: Point) -> i32 {
        if let Some(&res) = self.erosion_cache.borrow().get(&pos) {
            return res;
        }
        let geo_index = if pos == Z || pos == self.target {
            0
        } else if pos.y == 0 {
            pos.x * 16807
        } else if pos.x == 0 {
            pos.y * 48271
        } else {
            self.erosion(pos + W) * self.erosion(pos + N)
        };
        let res = (geo_index + self.depth) % 20183;
        self.erosion_cache.borrow_mut().insert(pos, res);
        res
    }
}

fn parse(input: &str) -> Cave {
    let (depth, target) = input.split_once('\n').unwrap();
    Cave {
        depth: depth.strip_prefix("depth: ").unwrap().parse().unwrap(),
        target: target.strip_prefix("target: ").unwrap().into(),
        erosion_cache: RefCell::new(HashMap::new()),
    }
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct State {
    pos: Point,
    tool: Tool,
    mins: i32,
}

impl State {
    fn adjacent<'a>(&'a self, cave: &'a Cave) -> impl Iterator<Item = Self> + 'a {
        self.pos
            .adjacent4()
            .into_iter()
            .filter_map(|pos| {
                if pos.x >= 0 && pos.y >= 0 && compatible(cave.erosion(pos).into(), self.tool) {
                    Some(State {
                        pos,
                        mins: self.mins + 1,
                        ..self.clone()
                    })
                } else {
                    None
                }
            })
            .chain(TOOLS.iter().filter_map(|&tool| {
                if tool != self.tool && compatible(cave.erosion(self.pos).into(), tool) {
                    Some(State {
                        tool,
                        mins: self.mins + 7,
                        ..self.clone()
                    })
                } else {
                    None
                }
            }))
    }
}

pub fn part1(input: &str) -> i32 {
    let cave = parse(input);
    let mut res = 0;
    for y in 0..=cave.target.y {
        for x in 0..=cave.target.x {
            res += cave.erosion(Point { y, x }) % 3;
        }
    }
    res
}

pub fn part2(input: &str) -> i32 {
    let cave = parse(input);
    let mut q = search2::a_star(
        State {
            pos: Z,
            tool: Tool::Torch,
            mins: 0,
        },
        |state| (state.pos, state.tool),
        |state| state.mins,
        |state| (cave.target - state.pos).manhattan(),
    );
    while let Some(state) = q.pop() {
        if state.pos == cave.target && state.tool == Tool::Torch {
            return state.mins;
        }
        for s in state.adjacent(&cave) {
            q.push(s);
        }
    }
    unreachable!()
}

pub fn tests() {
    let example = "depth: 510\ntarget: 10,10";
    assert_eq!(parse(example).erosion(Point { y: 1, x: 1 }), 1805);
    assert_eq!(part1(example), 114);
    assert_eq!(part2(example), 45);
}
