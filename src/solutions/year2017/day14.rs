use std::collections::HashSet;

use crate::{
    grid::{Point, Rect},
    hash, search,
};

#[derive(Clone)]
struct State<'a> {
    disk: &'a Rect<bool>,
    pos: Point,
}

impl<'a> search::State for State<'a> {
    type HashKey = Point;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(
            self.pos
                .adjacent4()
                .into_iter()
                .filter(|&pos| *self.disk.get(pos).unwrap_or(&false))
                .map(|pos| State {
                    disk: self.disk,
                    pos,
                }),
        )
    }

    fn hash_key(&self) -> Self::HashKey {
        self.pos
    }
}

fn disk(input: &str) -> Rect<bool> {
    let mut res = Rect::new(false, Point { x: 128, y: 128 });
    for y in 0..128 {
        let row = hash::knot(&format!("{input}-{y}"));
        for x in 0..128 {
            if row & (1 << x) != 0 {
                res[Point { x, y }] = true;
            }
        }
    }
    res
}

pub fn part1(input: &str) -> usize {
    disk(input).values().filter(|v| **v).count()
}

pub fn part2(input: &str) -> usize {
    let disk = disk(input);

    let mut regions = 0;
    let mut unexplored: HashSet<_> = disk.keys().filter(|pos| disk[pos]).collect();

    while !unexplored.is_empty() {
        regions += 1;
        for State { pos, .. } in search::depth_first(State {
            disk: &disk,
            pos: *unexplored.iter().next().unwrap(),
        }) {
            unexplored.remove(&pos);
        }
    }

    regions
}

pub fn tests() {
    assert_eq!(part1("flqrgnkx"), 8108);
    assert_eq!(part2("flqrgnkx"), 1242);
}
