use std::cmp;

fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let (low, high) = line.split_once('-').unwrap();
            (low.parse().unwrap(), high.parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut blacklist = parse(input);
    blacklist.sort_unstable();

    let mut lowest = 0;
    for (low, high) in blacklist {
        if lowest < low {
            return lowest;
        }
        lowest = high + 1;
    }
    unreachable!()
}

fn part2_(max: u32, input: &str) -> u32 {
    let mut blacklist = parse(input);
    blacklist.sort_unstable();

    let mut lowest = 0;
    let mut count = 0;
    for (low, high) in blacklist {
        if lowest < low {
            count += low - lowest;
        }
        if high == max {
            return count;
        }
        lowest = cmp::max(lowest, high + 1);
    }
    max - lowest + 1 + count
}

pub fn part2(input: &str) -> u32 {
    part2_(u32::MAX, input)
}

pub fn tests() {
    assert_eq!(part1("5-8\n0-2\n4-7"), 3);
    assert_eq!(part2_(9, "5-8\n0-2\n4-7"), 2);
}
