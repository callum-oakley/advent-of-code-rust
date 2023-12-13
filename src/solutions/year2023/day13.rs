use crate::grid::{Point, Rect};

fn parse(input: &str) -> impl Iterator<Item = Rect<bool>> + '_ {
    input
        .split("\n\n")
        .map(|pattern| Rect::parse(pattern, |_, c| c == '#'))
}

fn ver_asymmetry(pattern: &Rect<bool>, x: i32) -> usize {
    let mut before = x - 1;
    let mut after = x;
    let mut res = 0;
    while before >= 0 && after < pattern.size.x {
        for y in 0..pattern.size.y {
            if pattern[Point { y, x: before }] != pattern[Point { y, x: after }] {
                res += 1;
            }
        }
        before -= 1;
        after += 1;
    }
    res
}

fn hor_asymmetry(pattern: &Rect<bool>, y: i32) -> usize {
    let mut before = y - 1;
    let mut after = y;
    let mut res = 0;
    while before >= 0 && after < pattern.size.y {
        for x in 0..pattern.size.x {
            if pattern[Point { y: before, x }] != pattern[Point { y: after, x }] {
                res += 1;
            }
        }
        before -= 1;
        after += 1;
    }
    res
}

fn part_(target_asymmetry: usize, input: &str) -> i32 {
    parse(input)
        .map(|pattern| {
            for x in 1..pattern.size.x {
                if ver_asymmetry(&pattern, x) == target_asymmetry {
                    return x;
                }
            }
            for y in 1..pattern.size.y {
                if hor_asymmetry(&pattern, y) == target_asymmetry {
                    return 100 * y;
                }
            }
            unreachable!()
        })
        .sum()
}

pub fn part1(input: &str) -> i32 {
    part_(0, input)
}

pub fn part2(input: &str) -> i32 {
    part_(1, input)
}

pub fn tests() {
    let example = [
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]
    .join("\n");
    assert_eq!(part1(&example), 405);
    assert_eq!(part2(&example), 400);
}
