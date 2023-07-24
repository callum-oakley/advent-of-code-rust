use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use crate::grid::{Point, Rect};

struct Claim {
    id: u32,
    pos: Point,
    size: Point,
}

impl From<&str> for Claim {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+,\d+): (\d+x\d+)").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        Claim {
            id: captures[1].parse().unwrap(),
            pos: captures[2].into(),
            size: captures[3].into(),
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut res = 0;
    let mut fabric = Rect::new(0, Point { x: 1000, y: 1000 });
    for claim in input.lines().map(Claim::from) {
        for x in claim.pos.x..claim.pos.x + claim.size.x {
            for y in claim.pos.y..claim.pos.y + claim.size.y {
                let p = Point { x, y };
                fabric[p] += 1;
                if fabric[p] == 2 {
                    res += 1;
                }
            }
        }
    }
    res
}

pub fn part2(input: &str) -> u32 {
    let mut disjoint = HashSet::new();
    let mut fabric = Rect::new(None, Point { x: 1000, y: 1000 });
    for claim in input.lines().map(Claim::from) {
        disjoint.insert(claim.id);
        for x in claim.pos.x..claim.pos.x + claim.size.x {
            for y in claim.pos.y..claim.pos.y + claim.size.y {
                let p = Point { x, y };
                if let Some(id) = fabric[p] {
                    disjoint.remove(&id);
                    disjoint.remove(&claim.id);
                }
                fabric[p] = Some(claim.id);
            }
        }
    }
    assert_eq!(disjoint.len(), 1);
    disjoint.into_iter().next().unwrap()
}

pub fn tests() {
    let example = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    assert_eq!(part1(example), 4);
    assert_eq!(part2(example), 3);
}
