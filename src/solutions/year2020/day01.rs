use crate::combinatorics;

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_(n: usize, input: &str) -> u32 {
    combinatorics::combinations(n, &parse(input))
        .find(|combination| combination.iter().copied().sum::<u32>() == 2020)
        .unwrap()
        .into_iter()
        .product()
}

pub fn part1(input: &str) -> u32 {
    part_(2, input)
}

pub fn part2(input: &str) -> u32 {
    part_(3, input)
}

pub fn tests() {
    let example = "1721\n979\n366\n299\n675\n1456";
    assert_eq!(part1(example), 514_579);
    assert_eq!(part2(example), 241_861_950);
}
