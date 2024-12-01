use std::iter;

fn steps(subject_number: u64) -> impl Iterator<Item = u64> {
    iter::successors(Some(1), move |value| {
        Some((value * subject_number) % 20_201_227)
    })
}

fn loop_size(public_key: u64) -> usize {
    steps(7)
        .enumerate()
        .find(|&(_, value)| value == public_key)
        .unwrap()
        .0
}

pub fn part1(input: &str) -> u64 {
    let keys: Vec<u64> = input
        .split_whitespace()
        .map(|key| key.parse().unwrap())
        .collect();
    steps(keys[0]).nth(loop_size(keys[1])).unwrap()
}

pub fn tests() {
    assert_eq!(loop_size(5_764_801), 8);
    assert_eq!(loop_size(17_807_724), 11);
    assert_eq!(part1("5764801 17807724"), 14_897_079);
}
