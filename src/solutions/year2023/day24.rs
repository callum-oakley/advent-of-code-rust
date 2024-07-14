use nalgebra::{Vector2, Vector3};

use crate::{combinatorics, grid::IntoVector};

#[derive(Clone, Copy)]
struct Stone {
    p: Vector3<f64>,
    v: Vector3<f64>,
}

fn xy_intersection(s1: Stone, s2: Stone) -> Option<(f64, f64, Vector2<f64>)> {
    // Using the equations from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
    // re-arrange s = p + t * v to s = p + t * (v + p - p)
    let x1 = s1.p.x;
    let y1 = s1.p.y;
    let x2 = (s1.v + s1.p).x;
    let y2 = (s1.v + s1.p).y;
    let x3 = s2.p.x;
    let y3 = s2.p.y;
    let x4 = (s2.v + s2.p).x;
    let y4 = (s2.v + s2.p).y;

    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denominator == 0. {
        return None;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

    Some((t, u, Vector2::new(x1 + t * (x2 - x1), y1 + t * (y2 - y1))))
}

fn parse(input: &str) -> impl Iterator<Item = Stone> + '_ {
    input.trim().lines().map(|l| {
        let (p, v) = l.split_once('@').unwrap();
        Stone {
            p: p.into_vector(),
            v: v.into_vector(),
        }
    })
}

fn part1_(low: f64, high: f64, input: &str) -> usize {
    let stones = parse(input).collect::<Vec<_>>();
    combinatorics::combination(2, &stones)
        .filter_map(|pair| xy_intersection(*pair[0], *pair[1]))
        .filter(|&(t, u, p)| {
            t >= 0. && u >= 0. && low <= p.x && p.x <= high && low <= p.y && p.y <= high
        })
        .count()
}

pub fn part1(input: &str) -> usize {
    part1_(2e14, 4e14, input)
}

pub fn tests() {
    let example = "
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    ";
    assert_eq!(part1_(7., 27., example), 2);
}
