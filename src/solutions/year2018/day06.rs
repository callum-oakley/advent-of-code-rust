use std::collections::HashMap;

use crate::grid::{Bounds, Point};

fn unique_closest(pos: Point, coordinates: &[Point]) -> Option<Point> {
    let mut min_dist = (pos - coordinates[0]).manhattan();
    let mut res = Some(coordinates[0]);
    for &c in &coordinates[1..] {
        let dist = (pos - c).manhattan();
        match dist.cmp(&min_dist) {
            std::cmp::Ordering::Less => {
                min_dist = dist;
                res = Some(c);
            }
            std::cmp::Ordering::Equal => {
                res = None;
            }
            std::cmp::Ordering::Greater => {}
        }
    }
    res
}

pub fn part1(input: &str) -> u32 {
    let coordinates = input.lines().map(Point::from).collect::<Vec<_>>();

    let mut areas = coordinates
        .iter()
        .map(|c| (c, 0))
        .collect::<HashMap<_, _>>();

    let bounds = Bounds::new(coordinates.iter().copied());
    for x in bounds.min_x..=bounds.max_x {
        for y in bounds.min_y..=bounds.max_y {
            if let Some(c) = unique_closest(Point { y, x }, &coordinates) {
                if x == bounds.min_x || x == bounds.max_x || y == bounds.min_y || y == bounds.max_y
                {
                    // This area is inifinite, so we don't care about it.
                    areas.remove(&c);
                } else if let Some(v) = areas.get_mut(&c) {
                    *v += 1;
                }
            }
        }
    }

    areas.into_values().max().unwrap()
}

fn part2_(tolerance: i32, input: &str) -> u32 {
    let coordinates = input.lines().map(Point::from).collect::<Vec<_>>();

    let mut res = 0;

    let bounds = Bounds::new(coordinates.iter().copied());
    for x in bounds.min_x..=bounds.max_x {
        for y in bounds.min_y..=bounds.max_y {
            let pos = Point { y, x };
            if coordinates
                .iter()
                .map(|&c| (pos - c).manhattan())
                .sum::<i32>()
                < tolerance
            {
                res += 1;
            }
        }
    }

    res
}

pub fn part2(input: &str) -> u32 {
    part2_(10_000, input)
}

pub fn tests() {
    let example = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
    assert_eq!(part1(example), 17);
    assert_eq!(part2_(32, example), 16);
}
