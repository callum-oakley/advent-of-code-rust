#![warn(clippy::pedantic)]
#![cfg_attr(feature = "allow_dead_code", allow(dead_code))]

use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    time::{Duration, Instant},
};

use regex::Regex;

mod chinese_remainder;
mod combinatorics;
mod grid;
mod grid_3d;
mod grid_hex;
mod hash;
mod ocr;
mod part;
mod search;
mod solutions;
mod uniq;

fn sig_figs(n: u32, duration: Duration) -> Duration {
    let nanos = u64::try_from(duration.as_nanos()).unwrap();
    if nanos == 0 {
        return duration;
    }
    let magnitude = 10u64.pow(nanos.ilog10() - n + 1);
    Duration::from_nanos(nanos / magnitude * magnitude)
}

fn get(path: &str) -> reqwest::Result<String> {
    reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/{path}"))
        .header(
            "cookie",
            format!("session={}", fs::read_to_string(".session").unwrap().trim()),
        )
        .send()?
        .error_for_status()?
        .text()
}

fn get_input(year: u16, day: u8) -> String {
    let path = format!("input/{year}/{day:0>2}");
    let path = Path::new(&path);
    if path.exists() {
        fs::read_to_string(path).unwrap()
    } else {
        let input = get(&format!("{year}/day/{day}/input")).unwrap();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, &input).unwrap();
        input
    }
}

fn get_answer(year: u16, day: u8, part: u8) -> Option<String> {
    let path = format!("answer/{year}/{day:0>2}/{part}");
    let path = Path::new(&path);
    if path.exists() {
        Some(fs::read_to_string(path).unwrap())
    } else {
        let page = get(&format!("{year}/day/{day}")).unwrap();
        let mut answers: Vec<String> = Regex::new(r"Your puzzle answer was <code>([^<]*)")
            .unwrap()
            .captures_iter(&page)
            .map(|captures| captures[1].to_owned())
            .collect();
        for (i, answer) in answers.iter().enumerate() {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path.parent().unwrap().join((i + 1).to_string()), answer).unwrap();
        }
        if answers.len() >= part.into() {
            Some(answers.swap_remove(part as usize - 1))
        } else {
            None
        }
    }
}

fn run_part(year: u16, day: u8, part: u8, f: fn(&str) -> String, input: &str) -> Duration {
    let now = Instant::now();
    let answer = f(input);
    let elapsed = now.elapsed();

    let expected = get_answer(year, day, part);
    if let Some(ref expected) = expected {
        assert_eq!(&answer, expected);
    }

    println!(
        "{} {:0>2} {}   {: >5?}   {}{}",
        year,
        day,
        part,
        sig_figs(2, elapsed),
        answer,
        if expected.is_none() { " ?" } else { "" }
    );

    elapsed
}

fn run_day(year: u16, day: u8, solution: &solutions::Solution) -> (Duration, usize) {
    if let Some(tests) = solution.tests {
        tests();
    }

    let input = get_input(year, day);
    let input = input.trim_end_matches('\n');

    let mut elapsed = Duration::new(0, 0);
    let mut stars = 0;
    if let Some(part1) = solution.part1 {
        elapsed += run_part(year, day, 1, part1, input);
        stars += 1;
    }
    if let Some(part2) = solution.part2 {
        elapsed += run_part(year, day, 2, part2, input);
        stars += 1;
    }
    (elapsed, stars)
}

fn run_year(year: u16, year_solutions: &BTreeMap<u8, solutions::Solution>) -> (Duration, usize) {
    let mut elapsed = Duration::new(0, 0);
    let mut stars = 0;
    for (day, solution) in year_solutions {
        let (e, s) = run_day(year, *day, solution);
        elapsed += e;
        stars += s;
    }

    if stars == 49 {
        // day 25 part 2
        stars += 1;
    }

    println!("{:\u{2500}^80}", "");
    println!(
        "{} ** *   {: >5?}   {} / 50",
        year,
        sig_figs(2, elapsed),
        stars,
    );

    (elapsed, stars)
}

fn run(solutions: &BTreeMap<u16, BTreeMap<u8, solutions::Solution>>) {
    let mut elapsed = Duration::new(0, 0);
    let mut stars = 0;
    let mut available = 0;
    for (year, year_solutions) in solutions {
        let (e, s) = run_year(*year, year_solutions);
        elapsed += e;
        stars += s;
        available += 50;
        println!("{:\u{2550}^80}", "");
    }

    println!(
        "**** ** *   {: >5?}   {} / {}",
        sig_figs(2, elapsed),
        stars,
        available,
    );
    println!("{:\u{2500}^80}", "");
}

fn main() {
    println!("{:\u{2500}^80}", "");
    run(&solutions::build());
}
