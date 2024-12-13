use std::hash::Hash;

use crate::{
    grid::{self, Grid, Vector},
    search,
};

fn score<H, K>(grid: &Grid<u32>, v: Vector, hash_key: H) -> usize
where
    H: FnMut(&Vec<Vector>) -> K,
    K: Eq + Hash,
{
    search::breadth_first(vec![v], hash_key, |path, push| {
        let v = *path.last().unwrap();
        let height = grid[v];
        grid::adjacent4(v)
            .filter(|&v| grid.get(v).is_some_and(|&h| h == height + 1))
            .for_each(|v| {
                let mut path = path.clone();
                path.push(v);
                push(path);
            });
    })
    .filter(|path| grid[*path.last().unwrap()] == 9)
    .count()
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    grid.keys()
        .filter(|&v| grid[v] == 0)
        .map(|v| score(&grid, v, |path| *path.last().unwrap()))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    grid.keys()
        .filter(|&v| grid[v] == 0)
        .map(|v| score(&grid, v, Clone::clone))
        .sum()
}

pub fn tests() {
    let example = [
        "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
        "10456732",
    ]
    .join("\n");
    assert_eq!(part1(&example), 36);
    assert_eq!(part2(&example), 81);
}
