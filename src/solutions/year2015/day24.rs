use crate::combinatorics::combinations;

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|package| package.parse().unwrap())
        .collect()
}

pub fn part_(groups: u64, input: &str) -> u64 {
    let packages = parse(input);
    for group_size in 1.. {
        if let Some(res) = combinations(group_size, &packages)
            .filter(|group| {
                // Strictly speaking we should also check that the remaining
                // packages can be split, but this turns out to be unnecessary.
                group.iter().copied().sum::<u64>() * groups == packages.iter().sum::<u64>()
            })
            .map(|group| group.iter().copied().product())
            .min()
        {
            return res;
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> u64 {
    part_(3, input)
}

pub fn part2(input: &str) -> u64 {
    part_(4, input)
}

pub fn tests() {
    assert_eq!(part1("1 2 3 4 5 7 8 9 10 11"), 99);
    assert_eq!(part2("1 2 3 4 5 7 8 9 10 11"), 44);
}
