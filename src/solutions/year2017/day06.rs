use std::collections::HashMap;

use crate::part::Part;

fn redistribute(banks: &mut [u8]) {
    // max_by_key is last wins, and we want first wins, so rev
    let mut i = (0..banks.len()).rev().max_by_key(|i| banks[*i]).unwrap();
    let mut blocks = banks[i];
    banks[i] = 0;
    while blocks > 0 {
        blocks -= 1;
        i = (i + 1) % banks.len();
        banks[i] += 1;
    }
}

fn part_(part: Part, input: &str) -> usize {
    let mut banks = input
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect::<Vec<_>>();

    let mut seen = HashMap::new();
    let mut cycle = 0;

    loop {
        seen.insert(banks.clone(), cycle);
        redistribute(&mut banks);
        cycle += 1;
        if let Some(prev) = seen.get(&banks) {
            return match part {
                Part::One => cycle,
                Part::Two => cycle - prev,
            };
        }
    }
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    assert_eq!(part1("0 2 7 0"), 5);
    assert_eq!(part2("0 2 7 0"), 4);
}
