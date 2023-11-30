use std::collections::HashSet;

use crate::{
    grid::{Point, Rect},
    hash,
    search::{self, Queue},
};

fn disk(input: &str) -> Rect<bool> {
    let mut res = Rect::new(false, Point { x: 128, y: 128 });
    for y in 0..128 {
        let row = hash::knot(&format!("{input}-{y}"));
        for x in 0..128 {
            if row & (1 << x) != 0 {
                res[Point { y, x }] = true;
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
        let mut q = search::depth_first(*unexplored.iter().next().unwrap(), |&pos| pos);
        while let Some(pos) = q.pop() {
            unexplored.remove(&pos);
            for p in pos.adjacent4() {
                if *disk.get(p).unwrap_or(&false) {
                    q.push(p);
                }
            }
        }
    }

    regions
}

pub fn tests() {
    assert_eq!(part1("flqrgnkx"), 8108);
    assert_eq!(part2("flqrgnkx"), 1242);
}
