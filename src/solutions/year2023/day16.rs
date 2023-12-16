use std::collections::HashSet;

use crate::{
    grid::{Point, Rect, Turn, E, N, S, W, Z},
    search::{self, Queue},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Packet {
    pos: Point,
    dir: Point,
}

impl Packet {
    fn new(pos: Point, dir: Point) -> Self {
        Self { pos, dir }
    }

    fn tick(mut self) -> Self {
        self.pos += self.dir;
        self
    }

    fn turn(mut self, turn: Turn) -> Self {
        self.dir = self.dir.turn(turn);
        self
    }
}

fn energize(tiles: &Rect<char>, beam: Packet) -> usize {
    let mut energized = HashSet::new();
    let mut q = search::breadth_first(beam, |&p| p);
    while let Some(packet) = q.pop() {
        if tiles.get(packet.pos).is_none() {
            continue;
        }
        energized.insert(packet.pos);
        match (tiles[packet.pos], packet.dir) {
            ('.', _) | ('|', N | S) | ('-', E | W) => q.push(packet.tick()),
            ('\\', N | S) | ('/', E | W) => q.push(packet.turn(Turn::Left).tick()),
            ('\\', E | W) | ('/', N | S) => q.push(packet.turn(Turn::Right).tick()),
            ('|', E | W) | ('-', N | S) => {
                q.push(packet.turn(Turn::Left).tick());
                q.push(packet.turn(Turn::Right).tick());
            }
            _ => unreachable!(),
        }
    }
    energized.len()
}

pub fn part1(input: &str) -> usize {
    energize(&Rect::parse(input, |_, c| c), Packet::new(Z, E))
}

pub fn part2(input: &str) -> usize {
    let tiles = Rect::parse(input, |_, c| c);
    let size = tiles.size;
    let mut res = 0;
    for y in 0..size.y {
        res = res.max(energize(&tiles, Packet::new(Point::new(y, 0), E)));
        res = res.max(energize(&tiles, Packet::new(Point::new(y, size.x - 1), W)));
    }
    for x in 0..size.x {
        res = res.max(energize(&tiles, Packet::new(Point::new(0, x), S)));
        res = res.max(energize(&tiles, Packet::new(Point::new(size.y - 1, x), N)));
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
