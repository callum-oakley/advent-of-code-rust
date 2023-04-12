#![warn(clippy::pedantic)]

use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    time::{Duration, Instant},
};

use regex::Regex;

mod combinatorics;
mod grid;
mod solutions;
mod uniq;

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
        assert_eq!(expected, &answer);
    }

    println!(
        "{} {:0>2} {}   {: >5.0?}   {}{}",
        year,
        day,
        part,
        elapsed,
        answer,
        if expected.is_none() { " ?" } else { "" }
    );

    elapsed
}

fn run_day(year: u16, day: u8, solution: &solutions::Solution) -> Duration {
    if let Some(tests) = solution.tests {
        tests();
    }

    let input = get_input(year, day);
    let input = input.trim_end();

    let mut elapsed = Duration::new(0, 0);
    if let Some(part1) = solution.part1 {
        elapsed += run_part(year, day, 1, part1, input);
    }
    if let Some(part2) = solution.part2 {
        elapsed += run_part(year, day, 2, part2, input);
    }
    elapsed
}

fn run_year(year: u16, year_solutions: &BTreeMap<u8, solutions::Solution>) -> Duration {
    let mut elapsed = Duration::new(0, 0);
    for (day, solution) in year_solutions {
        elapsed += run_day(year, *day, solution);
    }

    println!("{:\u{2500}^80}", "");
    println!("{year} ** *   {elapsed: >5.0?}");

    elapsed
}

fn run(solutions: &BTreeMap<u16, BTreeMap<u8, solutions::Solution>>) {
    let mut elapsed = Duration::new(0, 0);
    for (year, year_solutions) in solutions {
        elapsed += run_year(*year, year_solutions);
        println!("{:\u{2550}^80}", "");
    }

    println!("**** ** *   {elapsed: >5.0?}");
    println!("{:\u{2500}^80}", "");
}

fn main() {
    println!("{:\u{2500}^80}", "");
    run(&solutions::build());
}
