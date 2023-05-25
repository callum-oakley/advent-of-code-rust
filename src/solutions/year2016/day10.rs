use std::collections::HashMap;

use regex::Regex;

enum Dest {
    Bot(u16),
    Bin(u16),
}

impl From<&str> for Dest {
    fn from(s: &str) -> Self {
        let mut words = s.split_whitespace();
        match words.next().unwrap() {
            "bot" => Dest::Bot(words.next().unwrap().parse().unwrap()),
            "output" => Dest::Bin(words.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

struct Bot {
    id: u16,
    low: Dest,
    high: Dest,
    chips: Vec<u16>,
}

fn parse(input: &str) -> HashMap<u16, Bot> {
    let mut bots = HashMap::new();

    let bot_re = Regex::new(r"bot (\d+) gives low to (\w+ \d+) and high to (\w+ \d+)").unwrap();
    for captures in bot_re.captures_iter(input) {
        let id = captures[1].parse().unwrap();
        bots.insert(
            id,
            Bot {
                id,
                low: captures[2].into(),
                high: captures[3].into(),
                chips: vec![],
            },
        );
    }

    let value_re = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    for captures in value_re.captures_iter(input) {
        let value = captures[1].parse().unwrap();
        let bot = captures[2].parse().unwrap();
        bots.get_mut(&bot).unwrap().chips.push(value);
    }

    bots
}

#[derive(Clone, Copy)]
enum ExitCondition {
    Part1 { target_low: u16, target_high: u16 },
    Part2,
}

fn part_(exit_condition: ExitCondition, input: &str) -> u16 {
    let mut bots = parse(input);
    let mut bins = HashMap::new();
    loop {
        let id = bots
            .values()
            .find(|bot| bot.chips.len() == 2)
            .expect("no bots have two chips")
            .id;

        let chips = &mut bots.get_mut(&id).unwrap().chips;
        chips.sort_unstable();
        let low = chips[0];
        let high = chips[1];
        chips.clear();

        match bots[&id].low {
            Dest::Bot(low_bot) => {
                bots.get_mut(&low_bot).unwrap().chips.push(low);
            }
            Dest::Bin(low_bin) => {
                bins.insert(low_bin, low);
            }
        }

        match bots[&id].high {
            Dest::Bot(high_bot) => {
                bots.get_mut(&high_bot).unwrap().chips.push(high);
            }
            Dest::Bin(high_bin) => {
                bins.insert(high_bin, high);
            }
        }

        match exit_condition {
            ExitCondition::Part1 {
                target_low,
                target_high,
            } => {
                if low == target_low && high == target_high {
                    return id;
                }
            }
            ExitCondition::Part2 => {
                if bins.contains_key(&0) && bins.contains_key(&1) && bins.contains_key(&2) {
                    return bins[&0] * bins[&1] * bins[&2];
                }
            }
        }
    }
}

pub fn part1(input: &str) -> u16 {
    part_(
        ExitCondition::Part1 {
            target_low: 17,
            target_high: 61,
        },
        input,
    )
}

pub fn part2(input: &str) -> u16 {
    part_(ExitCondition::Part2, input)
}

pub fn tests() {
    let example = "value 5 goes to bot 2
                   bot 2 gives low to bot 1 and high to bot 0
                   value 3 goes to bot 1
                   bot 1 gives low to output 1 and high to bot 0
                   bot 0 gives low to output 2 and high to output 0
                   value 2 goes to bot 2";
    assert_eq!(
        part_(
            ExitCondition::Part1 {
                target_low: 2,
                target_high: 5,
            },
            example,
        ),
        2,
    );
    assert_eq!(part2(example), 30);
}
