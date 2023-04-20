use std::collections::HashMap;

use regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    static ref HEAD_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref TAIL_RE: Regex = Regex::new(r"(\w+): (\d+)").unwrap();
    static ref TARGET_STATS: HashMap<&'static str, u16> = vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect();
}

struct Sue<'a> {
    id: u16,
    stats: HashMap<&'a str, u16>,
}

fn parse(input: &str) -> impl Iterator<Item = Sue> {
    input.lines().map(|line| {
        let (head, tail) = line.split_once(':').unwrap();
        Sue {
            id: HEAD_RE.find(head).unwrap().as_str().parse().unwrap(),
            stats: TAIL_RE
                .captures_iter(tail)
                .map(|captures| {
                    (
                        captures.get(1).unwrap().as_str(),
                        captures[2].parse().unwrap(),
                    )
                })
                .collect(),
        }
    })
}

pub fn part1(input: &str) -> u16 {
    parse(input)
        .find(|sue| sue.stats.iter().all(|(&k, &v)| v == TARGET_STATS[k]))
        .unwrap()
        .id
}

pub fn part2(input: &str) -> u16 {
    parse(input)
        .find(|sue| {
            sue.stats.iter().all(|(&k, &v)| match k {
                "cats" | "trees" => v > TARGET_STATS[k],
                "pomeranians" | "goldfish" => v < TARGET_STATS[k],
                _ => v == TARGET_STATS[k],
            })
        })
        .unwrap()
        .id
}
