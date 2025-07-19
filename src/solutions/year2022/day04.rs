use std::sync::LazyLock;

use regex::Regex;

fn parse(input: &str) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + '_ {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap());
    RE.captures_iter(input).map(|captures| {
        (
            (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        )
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|&((a, b), (c, d))| a <= c && d <= b || c <= a && b <= d)
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .filter(|&((a, b), (c, d))| a <= d && c <= b)
        .count()
}

pub fn tests() {
    let example = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    assert_eq!(part1(example), 2);
    assert_eq!(part2(example), 4);
}
