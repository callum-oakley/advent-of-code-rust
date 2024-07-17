use lazy_static::lazy_static;
use regex::Regex;

fn parse(input: &str) -> impl Iterator<Item = (usize, usize, char, &str)> + '_ {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    }
    RE.captures_iter(input).map(|captures| {
        (
            captures[1].parse().unwrap(),
            captures[2].parse().unwrap(),
            captures[3].chars().next().unwrap(),
            captures.get(4).unwrap().as_str(),
        )
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|&(low, high, letter, password)| {
            let count = password.chars().filter(|&c| c == letter).count();
            low <= count && count <= high
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .filter(|&(i, j, letter, password)| {
            let c = password.chars().collect::<Vec<char>>();
            c[i - 1] == letter && c[j - 1] != letter || c[i - 1] != letter && c[j - 1] == letter
        })
        .count()
}

pub fn tests() {
    let example = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    assert_eq!(part1(example), 2);
    assert_eq!(part2(example), 1);
}
