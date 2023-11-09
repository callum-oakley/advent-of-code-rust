use std::collections::{HashMap, HashSet};

use ordered_float::OrderedFloat;

use crate::grid::Point;

fn parse(input: &str) -> Vec<Point> {
    let mut res = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                res.push(Point {
                    y: y.try_into().unwrap(),
                    x: x.try_into().unwrap(),
                });
            }
        }
    }
    res
}

// Angle from the y-axis, in the range (-pi, pi] with zero straight down, and increasing clockwise.
// Note that this is shifted from the usual definition used in polar coordinates because it makes
// the problem easier.
fn theta(asteroid: Point) -> OrderedFloat<f64> {
    OrderedFloat(-f64::atan2(asteroid.x.into(), asteroid.y.into()))
}

fn count_visible(asteroids: &[Point], origin: Point) -> usize {
    asteroids
        .iter()
        .filter(|&&a| a != origin)
        .map(|&a| theta(a - origin))
        .collect::<HashSet<_>>()
        .len()
}

fn origin(asteroids: &[Point]) -> Point {
    asteroids
        .iter()
        .copied()
        .max_by_key(|&origin| count_visible(asteroids, origin))
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    let asteroids = parse(input);
    count_visible(&asteroids, origin(&asteroids))
}

pub fn part2(input: &str) -> i32 {
    let mut asteroids = parse(input);
    let origin = origin(&asteroids);

    let mut rays: HashMap<OrderedFloat<f64>, Vec<Point>> = HashMap::new();
    for &asteroid in &asteroids {
        rays.entry(theta(asteroid - origin))
            .or_default()
            .push(asteroid);
    }

    let mut depths: HashMap<Point, usize> = HashMap::new();
    for ray in rays.values_mut() {
        ray.sort_unstable_by_key(|&asteroid| (asteroid - origin).manhattan());
        for (depth, &asteroid) in ray.iter().enumerate() {
            depths.insert(asteroid, depth);
        }
    }

    asteroids.sort_unstable_by_key(|&asteroid| (depths[&asteroid], theta(asteroid - origin)));

    asteroids[199].x * 100 + asteroids[199].y
}

pub fn tests() {
    assert_eq!(part1(".#..#\n.....\n#####\n....#\n...##"), 8);
    assert_eq!(
        part1(
            &[
                "......#.#.",
                "#..#.#....",
                "..#######.",
                ".#.#.###..",
                ".#..#.....",
                "..#....#.#",
                "#..#....#.",
                ".##.#..###",
                "##...#..#.",
                ".#....####",
            ]
            .join("\n"),
        ),
        33,
    );
    assert_eq!(
        part1(
            &[
                "#.#...#.#.",
                ".###....#.",
                ".#....#...",
                "##.#.#.#.#",
                "....#.#.#.",
                ".##..###.#",
                "..#...##..",
                "..##....##",
                "......#...",
                ".####.###.",
            ]
            .join("\n"),
        ),
        35,
    );
    assert_eq!(
        part1(
            &[
                ".#..#..###",
                "####.###.#",
                "....###.#.",
                "..###.##.#",
                "##.##.#.#.",
                "....###..#",
                "..#.#..#.#",
                "#..#.#.###",
                ".##...##.#",
                ".....#.#..",
            ]
            .join("\n"),
        ),
        41,
    );

    let big_example = &[
        ".#..##.###...#######",
        "##.############..##.",
        ".#.######.########.#",
        ".###.#######.####.#.",
        "#####.##.#.##.###.##",
        "..#####..#.#########",
        "####################",
        "#.####....###.#.#.##",
        "##.#################",
        "#####.##.###..####..",
        "..######..##.#######",
        "####.##.####...##..#",
        ".#####..#.######.###",
        "##...#.##########...",
        "#.##########.#######",
        ".####.#.###.###.#.##",
        "....##.##.###..#####",
        ".#.#.###########.###",
        "#.#.#.#####.####.###",
        "###.##.####.##.#..##",
    ]
    .join("\n");
    assert_eq!(part1(big_example), 210);
    assert_eq!(part2(big_example), 802);
}
