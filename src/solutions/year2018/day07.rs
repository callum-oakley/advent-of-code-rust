use std::collections::HashMap;

use regex::bytes::Regex;

fn parse(input: &str) -> HashMap<char, Vec<char>> {
    let mut deps: HashMap<char, Vec<char>> = HashMap::new();
    for captures in Regex::new(r"Step (.) must be finished before step (.) can begin")
        .unwrap()
        .captures_iter(input.as_bytes())
    {
        let a = captures[1][0].into();
        let b = captures[2][0].into();
        deps.entry(a).or_default();
        deps.entry(b).or_default().push(a);
    }
    deps
}

fn next(deps: &HashMap<char, Vec<char>>, done: &str) -> Option<char> {
    deps.keys()
        .copied()
        .filter(|step| deps[step].iter().all(|&s| done.contains(s)))
        .min()
}

fn time(base_time: u32, step: char) -> u32 {
    base_time + step as u32 - 64
}

fn part2_(workers: usize, base_time: u32, input: &str) -> u32 {
    let mut deps = parse(input);
    let mut done = String::new();
    let mut in_progress: Vec<(char, u32)> = Vec::new();
    let mut t = 0;

    while !deps.is_empty() || !in_progress.is_empty() {
        while in_progress.len() < workers {
            if let Some(step) = next(&deps, &done) {
                deps.remove(&step);
                in_progress.push((step, time(base_time, step)));
            } else {
                break;
            }
        }

        let mut next_in_progress = Vec::new();
        let jump = in_progress.iter().map(|&(_, timer)| timer).min().unwrap();
        for (step, timer) in in_progress {
            if timer > jump {
                next_in_progress.push((step, timer - jump));
            } else {
                done.push(step);
            }
        }

        in_progress = next_in_progress;
        t += jump;
    }

    t
}

pub fn part1(input: &str) -> String {
    let mut deps = parse(input);
    let mut done = String::new();
    while let Some(step) = next(&deps, &done) {
        deps.remove(&step);
        done.push(step);
    }
    done
}

pub fn part2(input: &str) -> u32 {
    part2_(5, 60, input)
}

pub fn tests() {
    let example = "Step C must be finished before step A can begin.
                   Step C must be finished before step F can begin.
                   Step A must be finished before step B can begin.
                   Step A must be finished before step D can begin.
                   Step B must be finished before step E can begin.
                   Step D must be finished before step E can begin.
                   Step F must be finished before step E can begin.";
    assert_eq!(part1(example), "CABDFE");
    assert_eq!(part2_(2, 0, example), 15);
}
