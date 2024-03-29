use regex::Regex;

use crate::{
    grid_3d::{Axis, Bounds, Point},
    search::{self, Queue},
};

#[derive(Copy, Clone)]
struct Bot {
    pos: Point,
    r: i32,
}

fn parse(input: &str) -> Vec<Bot> {
    let re = Regex::new(r"pos=<([^>]+)>, r=(\d+)").unwrap();
    re.captures_iter(input)
        .map(|captures| Bot {
            pos: captures[1].into(),
            r: captures[2].parse().unwrap(),
        })
        .collect()
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Cube {
    pos: Point,
    w: i32,
}

impl Cube {
    // Does the given cube intersect with the given bot's range?
    fn intersects(self, bot: Bot) -> bool {
        let mut dist = 0;
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if bot.pos[axis] < self.pos[axis] {
                dist += self.pos[axis] - bot.pos[axis];
            } else if bot.pos[axis] > self.pos[axis] + self.w - 1 {
                dist += bot.pos[axis] - (self.pos[axis] + self.w - 1);
            }
        }
        dist <= bot.r
    }

    fn subcubes(&self) -> impl Iterator<Item = Self> + '_ {
        [0, self.w / 2].into_iter().flat_map(move |z| {
            [0, self.w / 2].into_iter().flat_map(move |y| {
                [0, self.w / 2].into_iter().map(move |x| Self {
                    pos: self.pos + Point { z, y, x },
                    w: self.w / 2,
                })
            })
        })
    }
}

pub fn part1(input: &str) -> usize {
    let bots = parse(input);
    let best_bot = bots.iter().max_by_key(|bot| bot.r).unwrap();
    bots.iter()
        .filter(|bot| (bot.pos - best_bot.pos).manhattan() <= best_bot.r)
        .count()
}

// We can hammer this puzzle in to a dijkstra shaped hole. Start with a cube containing every bot.
// We can take a "path" to a single coordinate by repeatedly splitting this cube in to 8 smaller
// cubes of half the width, and choosing one. Let the "cost" of a cube be the number of bots which
// DON'T intersect with it, then this cost always increases along any path, and the problem reduces
// to finding a lowest cost path to a cube of width 1.
pub fn part2(input: &str) -> i32 {
    let bots = parse(input);
    let bounds = Bounds::new(bots.iter().map(|bot| bot.pos));
    let mut q = search::dijkstra(
        Cube {
            pos: Point {
                z: bounds.min_z,
                y: bounds.min_y,
                x: bounds.min_x,
            },
            w: (bounds.max_x - bounds.min_x)
                .max(bounds.max_y - bounds.min_y)
                .max(bounds.max_z - bounds.min_z),
        },
        Clone::clone,
        |cube| {
            (
                bots.iter().filter(|&&bot| !cube.intersects(bot)).count(),
                // tie break with manhattan distance
                cube.pos.manhattan(),
            )
        },
    );
    while let Some(cube) = q.pop() {
        if cube.w == 1 {
            return cube.pos.manhattan();
        }
        for subcube in cube.subcubes() {
            q.push(subcube);
        }
    }
    unreachable!()
}

pub fn tests() {
    assert_eq!(
        part1(
            "pos=<0,0,0>, r=4\npos=<1,0,0>, r=1\npos=<4,0,0>, r=3
             pos=<0,2,0>, r=1\npos=<0,5,0>, r=3\npos=<0,0,3>, r=1
             pos=<1,1,1>, r=1\npos=<1,1,2>, r=1\npos=<1,3,1>, r=1"
        ),
        7,
    );
    assert_eq!(
        part2(
            "pos=<10,12,12>, r=2\npos=<12,14,12>, r=2\npos=<16,12,12>, r=4
             pos=<14,14,14>, r=6\npos=<50,50,50>, r=200\npos=<10,10,10>, r=5"
        ),
        36,
    );
}
