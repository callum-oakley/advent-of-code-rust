use regex::Regex;

use crate::{
    grid_3d::{Bounds, Point},
    search,
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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Cube {
    pos: Point,
    w: i32,
}

impl Cube {
    // Does the given cube intersect with the given bot's range?
    fn intersects(self, bot: Bot) -> bool {
        let mut dist = 0;
        for (a, low, high) in [
            (bot.pos.x, self.pos.x, self.pos.x + self.w - 1),
            (bot.pos.y, self.pos.y, self.pos.y + self.w - 1),
            (bot.pos.z, self.pos.z, self.pos.z + self.w - 1),
        ] {
            if a < low {
                dist += low - a;
            } else if a > high {
                dist += a - high;
            }
        }
        dist <= bot.r
    }
}

#[derive(Clone)]
struct State<'a> {
    cube: Cube,
    bots: &'a [Bot],
}

impl<'a> search::State for State<'a> {
    type HashKey = Cube;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new([0, self.cube.w / 2].into_iter().flat_map(move |z| {
            [0, self.cube.w / 2].into_iter().flat_map(move |y| {
                [0, self.cube.w / 2].into_iter().map(move |x| {
                    let mut state = self.clone();
                    state.cube.pos += Point { z, y, x };
                    state.cube.w /= 2;
                    state
                })
            })
        }))
    }

    fn hash_key(&self) -> Self::HashKey {
        self.cube
    }
}

impl<'a> search::OrdKey for State<'a> {
    type OrdKey = (usize, i32);

    fn ord_key(&self) -> Self::OrdKey {
        (
            self.bots
                .iter()
                .filter(|&&bot| !self.cube.intersects(bot))
                .count(),
            // tie break with manhattan distance
            self.cube.pos.manhattan(),
        )
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
    search::min_first(State {
        cube: Cube {
            pos: Point {
                z: bounds.min_z,
                y: bounds.min_y,
                x: bounds.min_x,
            },
            w: (bounds.max_x - bounds.min_x)
                .max(bounds.max_y - bounds.min_y)
                .max(bounds.max_z - bounds.min_z),
        },
        bots: &bots,
    })
    .find(|state| state.cube.w == 1)
    .unwrap()
    .cube
    .pos
    .manhattan()
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
