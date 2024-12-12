use std::collections::HashMap;

fn blink(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut next = HashMap::new();
    for (&stone, &freq) in stones {
        let s = stone.to_string();
        if stone == 0 {
            *next.entry(1).or_default() += freq;
        } else if s.len() % 2 == 0 {
            *next.entry(s[0..s.len() / 2].parse().unwrap()).or_default() += freq;
            *next.entry(s[s.len() / 2..].parse().unwrap()).or_default() += freq;
        } else {
            *next.entry(stone * 2024).or_default() += freq;
        }
    }
    next
}

fn part_(blinks: usize, input: &str) -> usize {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    for stone in input.split_whitespace().map(|s| s.parse().unwrap()) {
        *stones.entry(stone).or_default() += 1;
    }
    for _ in 0..blinks {
        stones = blink(&stones);
    }
    stones.values().sum()
}

pub fn part1(input: &str) -> usize {
    part_(25, input)
}

pub fn part2(input: &str) -> usize {
    part_(75, input)
}

pub fn tests() {
    assert_eq!(part1("125 17"), 55312);
}
