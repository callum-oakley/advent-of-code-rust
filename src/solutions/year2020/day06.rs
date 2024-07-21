use std::collections::HashSet;

fn part_<F>(input: &str, mut f: F) -> usize
where
    F: FnMut(HashSet<char>, HashSet<char>) -> HashSet<char>,
{
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect())
                .reduce(&mut f)
                .unwrap()
                .len()
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    part_(input, |a, b| a.union(&b).copied().collect())
}

pub fn part2(input: &str) -> usize {
    part_(input, |a, b| a.intersection(&b).copied().collect())
}

pub fn tests() {
    let example = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    assert_eq!(part1(example), 11);
    assert_eq!(part2(example), 6);
}
