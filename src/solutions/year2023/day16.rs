use std::collections::HashSet;

use crate::{
    grid2::{Grid, IntoChar, Turn, Vector, E, LEFT, N, RIGHT, S, W, Z},
    search::{self, Queue},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Packet {
    pos: Vector,
    dir: Vector,
}

impl Packet {
    fn new(pos: Vector, dir: Vector) -> Self {
        Self { pos, dir }
    }

    fn tick(mut self) -> Self {
        self.pos += self.dir;
        self
    }

    fn turn(mut self, turn: Turn) -> Self {
        self.dir = turn * self.dir;
        self
    }
}

fn energize(tiles: &Grid<char>, beam: Packet) -> usize {
    let mut energized = HashSet::new();
    let mut q = search::breadth_first(beam, |&p| p);
    while let Some(packet) = q.pop() {
        if tiles.get(packet.pos).is_none() {
            continue;
        }
        energized.insert(packet.pos);
        match (tiles[packet.pos], packet.dir.into_char()) {
            ('.', _) | ('|', 'N' | 'S') | ('-', 'E' | 'W') => q.push(packet.tick()),
            ('\\', 'N' | 'S') | ('/', 'E' | 'W') => q.push(packet.turn(LEFT).tick()),
            ('\\', 'E' | 'W') | ('/', 'N' | 'S') => q.push(packet.turn(RIGHT).tick()),
            ('|', 'E' | 'W') | ('-', 'N' | 'S') => {
                q.push(packet.turn(LEFT).tick());
                q.push(packet.turn(RIGHT).tick());
            }
            _ => unreachable!(),
        }
    }
    energized.len()
}

pub fn part1(input: &str) -> usize {
    energize(&Grid::parse(input, |_, c| c), Packet::new(Z, E))
}

pub fn part2(input: &str) -> usize {
    let tiles = Grid::parse(input, |_, c| c);
    let size = tiles.size;
    let mut res = 0;
    for y in 0..size.y {
        res = res.max(energize(&tiles, Packet::new(Vector::new(0, y), E)));
        res = res.max(energize(&tiles, Packet::new(Vector::new(size.x - 1, y), W)));
    }
    for x in 0..size.x {
        res = res.max(energize(&tiles, Packet::new(Vector::new(x, 0), S)));
        res = res.max(energize(&tiles, Packet::new(Vector::new(x, size.y - 1), N)));
    }
    res
}

pub fn tests() {
    let example = [
        r".|...\....",
        r"|.-.\.....",
        r".....|-...",
        r"........|.",
        r"..........",
        r".........\",
        r"..../.\\..",
        r".-.-/..|..",
        r".|....-|.\",
        r"..//.|....",
    ]
    .join("\n");
    assert_eq!(part1(&example), 46);
    assert_eq!(part2(&example), 51);
}
