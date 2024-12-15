use std::mem;

use crate::grid::{self, Grid, Vector, Z};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

fn part_<F>(input: &str, tolerance: usize, mut adjacent: F) -> usize
where
    F: FnMut(&Grid<Tile>, Vector) -> usize,
{
    let mut state = Grid::parse(input, |_, c| match c {
        '.' => Tile::Floor,
        'L' => Tile::Empty,
        '#' => Tile::Occupied,
        _ => unreachable!(),
    });
    let mut scrap = Grid::new(Tile::Floor, state.size);
    while state != scrap {
        for (pos, &tile) in &state {
            let adj = adjacent(&state, pos);
            if tile == Tile::Empty && adj == 0 {
                scrap[pos] = Tile::Occupied;
            } else if tile == Tile::Occupied && adj >= tolerance {
                scrap[pos] = Tile::Empty;
            } else {
                scrap[pos] = state[pos];
            }
        }
        mem::swap(&mut scrap, &mut state);
    }
    state.into_values().filter(|&t| t == Tile::Occupied).count()
}

pub fn part1(input: &str) -> usize {
    part_(input, 4, |state, pos| {
        grid::adjacent8(pos)
            .filter(|&v| state.get(v).is_some_and(|&t| t == Tile::Occupied))
            .count()
    })
}

pub fn part2(input: &str) -> usize {
    part_(input, 5, |state, pos| {
        grid::adjacent8(Z)
            .filter(|&dir| {
                let mut pos = pos + dir;
                while state.get(pos) == Some(&Tile::Floor) {
                    pos += dir;
                }
                state.get(pos) == Some(&Tile::Occupied)
            })
            .count()
    })
}

pub fn tests() {
    let example = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ]
    .join("\n");
    assert_eq!(part1(&example), 37);
    assert_eq!(part2(&example), 26);
}
