use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    grid::{Point, Rect, E, S},
    ocr,
};

enum Instruction {
    Rect(Point),
    RotRow { y: i32, by: i32 },
    RotCol { x: i32, by: i32 },
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    input.lines().map(|line| {
        let line = line.trim();
        let mut nums = RE.find_iter(line);
        if line.starts_with("rotate row") {
            Instruction::RotRow {
                y: nums.next().unwrap().as_str().parse().unwrap(),
                by: nums.next().unwrap().as_str().parse().unwrap(),
            }
        } else if line.starts_with("rotate col") {
            Instruction::RotCol {
                x: nums.next().unwrap().as_str().parse().unwrap(),
                by: nums.next().unwrap().as_str().parse().unwrap(),
            }
        } else {
            Instruction::Rect(Point {
                x: nums.next().unwrap().as_str().parse().unwrap(),
                y: nums.next().unwrap().as_str().parse().unwrap(),
            })
        }
    })
}

fn reverse(screen: &mut Rect<bool>, dir: Point, mut start: Point, mut end: Point) {
    end -= dir;
    while start < end {
        let tmp = screen[start];
        screen[start] = screen[end];
        screen[end] = tmp;

        start += dir;
        end -= dir;
    }
}

fn block_swap(screen: &mut Rect<bool>, dir: Point, start: Point, mid: Point, end: Point) {
    reverse(screen, dir, start, mid);
    reverse(screen, dir, mid, end);
    reverse(screen, dir, start, end);
}

fn part_(size: Point, input: &str) -> Rect<bool> {
    let mut screen = Rect::new(false, size);
    for instruction in parse(input) {
        match instruction {
            Instruction::Rect(p) => {
                for x in 0..p.x {
                    for y in 0..p.y {
                        screen[Point { y, x }] = true;
                    }
                }
            }
            Instruction::RotRow { y, by } => block_swap(
                &mut screen,
                E,
                Point { x: 0, y },
                Point { x: size.x - by, y },
                Point { x: size.x, y },
            ),
            Instruction::RotCol { x, by } => block_swap(
                &mut screen,
                S,
                Point { x, y: 0 },
                Point { x, y: size.y - by },
                Point { x, y: size.y },
            ),
        }
    }
    screen
}

pub fn part1(input: &str) -> usize {
    part_(Point { x: 50, y: 6 }, input)
        .values()
        .filter(|p| **p)
        .count()
}

pub fn part2(input: &str) -> &str {
    ocr::parse(&part_(Point { x: 50, y: 6 }, input).to_string())
}

pub fn tests() {
    let example = "rect 3x2
                   rotate column x=1 by 1
                   rotate row y=0 by 4
                   rotate column x=1 by 1";
    assert_eq!(
        part_(Point { x: 7, y: 3 }, example).to_string(),
        ".#..#.#\n#.#....\n.#.....\n",
    );
}
