use std::collections::VecDeque;

use crate::{
    grid::{Grid, Vector},
    search::{self, Queue},
};

fn part_<F, Q>(search: F, input: &str) -> usize
where
    F: Fn(Vector) -> Q,
    Q: Queue<Item = Vector>,
{
    let grid = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    grid.iter()
        .filter(|&(_, &height)| height == 0)
        .map(|(v, _)| {
            let mut score = 0;
            let mut q = search(v);
            while let Some(v) = q.pop() {
                let height = grid[v];
                if height == 9 {
                    score += 1;
                }
                grid.adjacent4(v)
                    .filter(|&(_, &h)| h == height + 1)
                    .for_each(|(v, _)| q.push(v));
            }
            score
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    part_(|v| search::breadth_first(v, |&v| v), input)
}

pub fn part2(input: &str) -> usize {
    part_(|v| VecDeque::from([v]), input)
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
