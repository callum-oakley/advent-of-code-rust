use std::collections::HashMap;

use crate::freqs::Freqs;

fn tick(fish: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut next = HashMap::new();
    for (&timer, &freq) in fish {
        if timer == 0 {
            *next.entry(6).or_default() += freq;
            *next.entry(8).or_default() += freq;
        } else {
            *next.entry(timer - 1).or_default() += freq;
        }
    }
    next
}

fn part_(days: usize, input: &str) -> usize {
    let mut fish = input.split(',').map(|s| s.parse().unwrap()).freqs();
    for _ in 0..days {
        fish = tick(&fish);
    }
    fish.values().sum()
}

pub fn part1(input: &str) -> usize {
    part_(80, input)
}

pub fn part2(input: &str) -> usize {
    part_(256, input)
}

pub fn tests() {
    let example = "3,4,3,1,2";
    assert_eq!(part1(example), 5934);
    assert_eq!(part2(example), 26_984_457_539);
}
