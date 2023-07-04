use std::collections::HashMap;

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

fn part_(part: u8, input: &str) -> usize {
    let mut banks = input
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect::<Vec<_>>();

    let mut seen = HashMap::new();
    let mut cycle = 0;
    seen.insert(banks.clone(), cycle);

    loop {
        redistribute(&mut banks);
        cycle += 1;
        if let Some(prev) = seen.get(&banks) {
            if part == 2 {
                return cycle - prev;
            }
            return cycle;
        }
        seen.insert(banks.clone(), cycle);
    }
}

pub fn part1(input: &str) -> usize {
    part_(1, input)
}

pub fn part2(input: &str) -> usize {
    part_(2, input)
}

pub fn tests() {
    assert_eq!(part1("0 2 7 0"), 5);
    assert_eq!(part2("0 2 7 0"), 4);
}
