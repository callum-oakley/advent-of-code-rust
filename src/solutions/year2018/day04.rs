use std::{collections::HashMap, ops::Add, sync::LazyLock};

use regex::Regex;

enum Event {
    Begin(u32),
    Sleep,
    Wake,
}

struct Log {
    minute: u32,
    event: Event,
}

impl From<&str> for Log {
    fn from(s: &str) -> Self {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r":(\d\d)\] (\w+) #?(\d+)?").unwrap());
        let captures = RE.captures(s).unwrap();
        Log {
            minute: captures[1].parse().unwrap(),
            event: match &captures[2] {
                "Guard" => Event::Begin(captures[3].parse().unwrap()),
                "falls" => Event::Sleep,
                "wakes" => Event::Wake,
                _ => unreachable!(),
            },
        }
    }
}

fn part_(strategy: fn(u32, u32) -> u32, input: &str) -> u32 {
    let mut logs = input.lines().map(str::trim).collect::<Vec<_>>();
    logs.sort_unstable();

    // guard -> minute -> frequency
    let mut m: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    let mut guard = 0;
    let mut sleep = 0;
    for log in logs.into_iter().map(Log::from) {
        match log.event {
            Event::Begin(g) => {
                guard = g;
            }
            Event::Sleep => {
                sleep = log.minute;
            }
            Event::Wake => {
                let minute_to_freq = m.entry(guard).or_default();
                for minute in sleep..log.minute {
                    *minute_to_freq.entry(minute).or_default() += 1;
                }
            }
        }
    }

    let guard = *m
        .keys()
        .max_by_key(|&&guard| m[&guard].values().copied().reduce(strategy))
        .unwrap();
    let minute = *m[&guard]
        .keys()
        .max_by_key(|&&minute| m[&guard][&minute])
        .unwrap();
    guard * minute
}

pub fn part1(input: &str) -> u32 {
    part_(u32::add, input)
}

pub fn part2(input: &str) -> u32 {
    part_(u32::max, input)
}

pub fn tests() {
    let example = "[1518-11-01 00:00] Guard #10 begins shift
                   [1518-11-01 00:05] falls asleep
                   [1518-11-01 00:25] wakes up
                   [1518-11-01 00:30] falls asleep
                   [1518-11-01 00:55] wakes up
                   [1518-11-01 23:58] Guard #99 begins shift
                   [1518-11-02 00:40] falls asleep
                   [1518-11-02 00:50] wakes up
                   [1518-11-03 00:05] Guard #10 begins shift
                   [1518-11-03 00:24] falls asleep
                   [1518-11-03 00:29] wakes up
                   [1518-11-04 00:02] Guard #99 begins shift
                   [1518-11-04 00:36] falls asleep
                   [1518-11-04 00:46] wakes up
                   [1518-11-05 00:03] Guard #99 begins shift
                   [1518-11-05 00:45] falls asleep
                   [1518-11-05 00:55] wakes up";
    assert_eq!(part1(example), 240);
    assert_eq!(part2(example), 4455);
}
