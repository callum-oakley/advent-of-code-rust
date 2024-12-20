use std::collections::HashSet;

use crate::{
    grid::{Adjacent, Grid},
    hash, search,
};

fn disk(input: &str) -> Grid<bool> {
    let mut res = Grid::new(false, [128, 128]);
    for y in 0..128 {
        let row = hash::knot(&format!("{input}-{y}"));
        for x in 0..128 {
            if row & (1 << x) != 0 {
                res[[x, y]] = true;
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
    let mut unexplored: HashSet<_> = disk.keys().filter(|&pos| disk[pos]).collect();

    while !unexplored.is_empty() {
        regions += 1;
        for pos in search::breadth_first(
            *unexplored.iter().next().unwrap(),
            |&pos, push| {
                pos.adjacent4()
                    .filter(|&v| disk.get(v).is_some_and(|&used| used))
                    .for_each(push);
            },
            search::id_filter(),
        ) {
            unexplored.remove(&pos);
        }
    }

    regions
}

pub fn tests() {
    assert_eq!(part1("flqrgnkx"), 8108);
    assert_eq!(part2("flqrgnkx"), 1242);
}
