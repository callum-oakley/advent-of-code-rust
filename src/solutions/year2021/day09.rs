use std::cmp::Reverse;

use crate::{
    freqs::Freqs,
    grid::{Adjacent, Grid, Vector},
};

fn downhill(g: &Grid<u32>, v: Vector) -> Option<Vector> {
    v.adjacent4().find(|&a| g.get(a).is_some_and(|&h| h < g[v]))
}

// Identify basins with their low point, so for a given position, return the low point of the basin
// it's in.
fn basin(g: &Grid<u32>, v: Vector) -> Vector {
    downhill(g, v).map_or(v, |d| basin(g, d))
}

pub fn part1(input: &str) -> u32 {
    let g = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    g.keys()
        .filter(|&v| v.adjacent4().all(|a| g.get(a).is_none_or(|&h| h > g[v])))
        .map(|v| g[v] + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let g = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    let freqs = g
        .keys()
        .filter(|&v| g[v] != 9)
        .map(|v| basin(&g, v))
        .freqs();
    let mut basins: Vec<_> = freqs.keys().collect();
    basins.sort_by_key(|&v| Reverse(freqs[v]));
    basins.iter().take(3).map(|&v| freqs[v]).product()
}

pub fn tests() {
    let example = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
    assert_eq!(part1(example), 15);
    assert_eq!(part2(example), 1134);
}
