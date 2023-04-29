use std::collections::HashSet;

use crate::grid::{Turn, N, Z};

fn parse(input: &str) -> impl Iterator<Item = (Turn, i32)> + '_ {
    input
        .split(", ")
        .map(|instruction| (instruction[0..1].into(), instruction[1..].parse().unwrap()))
}

pub fn part1(input: &str) -> i32 {
    parse(input)
        .fold((Z, N), |(pos, dir), (turn, dist)| {
            let dir = dir.turn(turn);
            (pos + dir * dist, dir)
        })
        .0
        .manhattan()
}

pub fn part2(input: &str) -> i32 {
    let mut pos = Z;
    let mut dir = N;
    let mut visited = HashSet::from([Z]);
    for (turn, dist) in parse(input) {
        dir = dir.turn(turn);
        for _ in 0..dist {
            pos += dir;
            if visited.contains(&pos) {
                return pos.manhattan();
            }
            visited.insert(pos);
        }
    }
    unreachable!()
}

pub fn tests() {
    assert_eq!(part1("R2, L3"), 5);
    assert_eq!(part1("R2, R2, R2"), 2);
    assert_eq!(part1("R5, L5, R5, R3"), 12);

    assert_eq!(part2("R8, R4, R4, R8"), 4);
}
