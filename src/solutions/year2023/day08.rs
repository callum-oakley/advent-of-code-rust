use std::collections::HashMap;

use num::Integer;
use regex::Regex;

fn parse(input: &str) -> (&[u8], HashMap<&str, (&str, &str)>) {
    let re = Regex::new(r"[0-9A-Z]{3}").unwrap();
    let (instructions, network) = input.split_once("\n\n").unwrap();
    (
        instructions.as_bytes(),
        network
            .lines()
            .map(|line| {
                let mut elements = re.find_iter(line);
                (
                    elements.next().unwrap().as_str(),
                    (
                        elements.next().unwrap().as_str(),
                        elements.next().unwrap().as_str(),
                    ),
                )
            })
            .collect(),
    )
}

fn dist<'a>(
    instructions: &'a [u8],
    network: &HashMap<&'a str, (&'a str, &'a str)>,
    start: &'a str,
    end: impl Fn(&str) -> bool,
) -> usize {
    let mut element = start;
    let mut steps = 0;
    while !end(element) {
        element = match instructions[steps % instructions.len()] {
            b'L' => network[element].0,
            b'R' => network[element].1,
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
}

pub fn part1(input: &str) -> usize {
    let (instructions, network) = parse(input);
    dist(instructions, &network, "AAA", |e| e == "ZZZ")
}

pub fn part2(input: &str) -> usize {
    let (instructions, network) = parse(input);
    network
        .keys()
        .filter(|&e| &e[2..] == "A")
        // We always reach a Z, and then cycle with a period equal to the number of steps it took us
        // to get there.
        .map(|e| dist(instructions, &network, e, |e| &e[2..] == "Z"))
        .reduce(|x, y| Integer::lcm(&x, &y))
        .unwrap()
}

pub fn tests() {
    let example0 = [
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)",
    ]
    .join("\n");

    let example1 = [
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ]
    .join("\n");

    let example2 = [
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ]
    .join("\n");

    assert_eq!(part1(&example0), 2);
    assert_eq!(part1(&example1), 6);
    assert_eq!(part2(&example2), 6);
}
