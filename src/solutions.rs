// generated by build.rs -- don't edit directly

mod year2015 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

use std::collections::BTreeMap;

pub struct Solution {
    pub part1: Option<fn(&str) -> String>,
    pub part2: Option<fn(&str) -> String>,
    pub tests: Option<fn() -> ()>,
}

// Clippy doesn't like the redundant to_string when the solution is
// already a String.
#[allow(clippy::redundant_clone)]
#[allow(clippy::too_many_lines)]
pub fn build() -> BTreeMap<u16, BTreeMap<u8, Solution>> {
    let mut solutions: BTreeMap<u16, BTreeMap<u8, Solution>> = BTreeMap::new();

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        1,
        Solution {
            part1: Some(|input| year2015::day01::part1(input).to_string()),
            part2: Some(|input| year2015::day01::part2(input).to_string()),
            tests: Some(year2015::day01::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        2,
        Solution {
            part1: Some(|input| year2015::day02::part1(input).to_string()),
            part2: Some(|input| year2015::day02::part2(input).to_string()),
            tests: None,
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        3,
        Solution {
            part1: Some(|input| year2015::day03::part1(input).to_string()),
            part2: Some(|input| year2015::day03::part2(input).to_string()),
            tests: Some(year2015::day03::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        4,
        Solution {
            part1: Some(|input| year2015::day04::part1(input).to_string()),
            part2: Some(|input| year2015::day04::part2(input).to_string()),
            tests: Some(year2015::day04::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        5,
        Solution {
            part1: Some(|input| year2015::day05::part1(input).to_string()),
            part2: Some(|input| year2015::day05::part2(input).to_string()),
            tests: Some(year2015::day05::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        6,
        Solution {
            part1: Some(|input| year2015::day06::part1(input).to_string()),
            part2: Some(|input| year2015::day06::part2(input).to_string()),
            tests: None,
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        7,
        Solution {
            part1: Some(|input| year2015::day07::part1(input).to_string()),
            part2: Some(|input| year2015::day07::part2(input).to_string()),
            tests: Some(year2015::day07::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        8,
        Solution {
            part1: Some(|input| year2015::day08::part1(input).to_string()),
            part2: Some(|input| year2015::day08::part2(input).to_string()),
            tests: Some(year2015::day08::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        9,
        Solution {
            part1: Some(|input| year2015::day09::part1(input).to_string()),
            part2: Some(|input| year2015::day09::part2(input).to_string()),
            tests: Some(year2015::day09::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        10,
        Solution {
            part1: Some(|input| year2015::day10::part1(input).to_string()),
            part2: Some(|input| year2015::day10::part2(input).to_string()),
            tests: Some(year2015::day10::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        11,
        Solution {
            part1: Some(|input| year2015::day11::part1(input).to_string()),
            part2: Some(|input| year2015::day11::part2(input).to_string()),
            tests: Some(year2015::day11::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        12,
        Solution {
            part1: Some(|input| year2015::day12::part1(input).to_string()),
            part2: Some(|input| year2015::day12::part2(input).to_string()),
            tests: Some(year2015::day12::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        13,
        Solution {
            part1: Some(|input| year2015::day13::part1(input).to_string()),
            part2: Some(|input| year2015::day13::part2(input).to_string()),
            tests: Some(year2015::day13::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        14,
        Solution {
            part1: Some(|input| year2015::day14::part1(input).to_string()),
            part2: Some(|input| year2015::day14::part2(input).to_string()),
            tests: Some(year2015::day14::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        15,
        Solution {
            part1: Some(|input| year2015::day15::part1(input).to_string()),
            part2: Some(|input| year2015::day15::part2(input).to_string()),
            tests: Some(year2015::day15::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        16,
        Solution {
            part1: Some(|input| year2015::day16::part1(input).to_string()),
            part2: Some(|input| year2015::day16::part2(input).to_string()),
            tests: None,
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        17,
        Solution {
            part1: Some(|input| year2015::day17::part1(input).to_string()),
            part2: Some(|input| year2015::day17::part2(input).to_string()),
            tests: Some(year2015::day17::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        18,
        Solution {
            part1: Some(|input| year2015::day18::part1(input).to_string()),
            part2: Some(|input| year2015::day18::part2(input).to_string()),
            tests: Some(year2015::day18::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        19,
        Solution {
            part1: Some(|input| year2015::day19::part1(input).to_string()),
            part2: Some(|input| year2015::day19::part2(input).to_string()),
            tests: Some(year2015::day19::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        20,
        Solution {
            part1: Some(|input| year2015::day20::part1(input).to_string()),
            part2: Some(|input| year2015::day20::part2(input).to_string()),
            tests: Some(year2015::day20::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        21,
        Solution {
            part1: Some(|input| year2015::day21::part1(input).to_string()),
            part2: Some(|input| year2015::day21::part2(input).to_string()),
            tests: Some(year2015::day21::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        22,
        Solution {
            part1: Some(|input| year2015::day22::part1(input).to_string()),
            part2: Some(|input| year2015::day22::part2(input).to_string()),
            tests: None,
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        23,
        Solution {
            part1: Some(|input| year2015::day23::part1(input).to_string()),
            part2: Some(|input| year2015::day23::part2(input).to_string()),
            tests: None,
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        24,
        Solution {
            part1: Some(|input| year2015::day24::part1(input).to_string()),
            part2: Some(|input| year2015::day24::part2(input).to_string()),
            tests: Some(year2015::day24::tests),
        },
    );

    solutions.entry(2015).or_insert_with(BTreeMap::new).insert(
        25,
        Solution {
            part1: Some(|input| year2015::day25::part1(input).to_string()),
            part2: None,
            tests: None,
        },
    );

    solutions
}
