use std::collections::HashMap;

use crate::{
    grid::{self, Grid, Vector},
    search,
};

fn parse(input: &str) -> (Grid<bool>, Vector) {
    let mut start = None;
    let grid = Grid::parse(input, |v, c| match c {
        '.' | 'E' => true,
        '#' => false,
        'S' => {
            start = Some(v);
            true
        }
        _ => unreachable!(),
    });
    (grid, start.unwrap())
}

fn run(grid: &Grid<bool>, start: Vector) -> HashMap<Vector, i32> {
    search::breadth_first(
        (start, 0),
        |&(pos, picoseconds), push| {
            grid::adjacent4(pos)
                .filter(|&a| grid.get(a).is_some_and(|&t| t))
                .for_each(|a| {
                    push((a, picoseconds + 1));
                });
        },
        search::hash_filter(|&(pos, _)| pos),
    )
    .collect()
}

fn cheats(track: &HashMap<Vector, i32>, max_duration: i32) -> impl Iterator<Item = i32> + '_ {
    track
        .keys()
        .flat_map(|&a| track.keys().map(move |&b| (a, b, (b - a).abs().sum())))
        .filter(move |&(a, b, duration)| {
            duration <= max_duration && track[&b] > track[&a] + duration
        })
        .map(move |(a, b, duration)| track[&b] - track[&a] - duration)
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    cheats(&run(&grid, start), 2)
        .filter(|&saving| saving >= 100)
        .count()
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    cheats(&run(&grid, start), 20)
        .filter(|&saving| saving >= 100)
        .count()
}
