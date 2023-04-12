use std::cmp::min;

use lazy_static::lazy_static;
use regex::Regex;

struct Reindeer {
    speed: u32,
    stamina: u32,
    sleepyness: u32,
}

impl Reindeer {
    fn distance(&self, t: u32) -> u32 {
        let period = self.stamina + self.sleepyness;
        let time_flying = t / period * self.stamina + min(t % period, self.stamina);
        time_flying * self.speed
    }
}

fn parse(input: &str) -> impl Iterator<Item = Reindeer> + '_ {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds."
        )
        .unwrap();
    }
    RE.captures_iter(input).map(|captures| Reindeer {
        speed: captures[1].parse().unwrap(),
        stamina: captures[2].parse().unwrap(),
        sleepyness: captures[3].parse().unwrap(),
    })
}

fn part1_(max_t: u32, input: &str) -> u32 {
    parse(input).map(|r| r.distance(max_t)).max().unwrap()
}

fn part2_(max_t: u32, input: &str) -> u32 {
    let reindeer: Vec<_> = parse(input).collect();
    let mut points = vec![0; reindeer.len()];
    for t in 1..=max_t {
        let max_dist = reindeer.iter().map(|r| r.distance(t)).max().unwrap();
        for (i, r) in reindeer.iter().enumerate() {
            if r.distance(t) == max_dist {
                points[i] += 1;
            }
        }
    }
    points.into_iter().max().unwrap()
}

pub fn part1(input: &str) -> u32 {
    part1_(2503, input)
}

pub fn part2(input: &str) -> u32 {
    part2_(2503, input)
}

pub fn tests() {
    let example = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
                   Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(part1_(1000, example), 1120);
    assert_eq!(part2_(1000, example), 689);
}
