fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.trim().lines() {
        let mut ids = line.split_whitespace();
        left.push(ids.next().unwrap().parse().unwrap());
        right.push(ids.next().unwrap().parse().unwrap());
    }
    (left, right)
}

pub fn part1(input: &str) -> usize {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (left, right) = parse(input);
    left.iter()
        .map(|l| l * right.iter().filter(|&r| r == l).count())
        .sum()
}

pub fn tests() {
    let example = "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";
    assert_eq!(part1(example), 11);
    assert_eq!(part2(example), 31);
}
