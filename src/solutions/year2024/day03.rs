use std::sync::LazyLock;

use regex::Regex;

enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap());
    RE.captures_iter(input).map(|captures| match &captures[0] {
        "do()" => Instruction::Do,
        "don't()" => Instruction::Dont,
        _ => Instruction::Mul(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
    })
}

pub fn part1(input: &str) -> i32 {
    parse(input)
        .filter_map(|instruction| match instruction {
            Instruction::Mul(x, y) => Some(x * y),
            _ => None,
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    parse(input)
        .fold((0, true), |(acc, enabled), instruction| {
            match (instruction, enabled) {
                (Instruction::Mul(x, y), true) => (acc + x * y, true),
                (Instruction::Mul(_, _), false) | (Instruction::Dont, _) => (acc, false),
                (Instruction::Do, _) => (acc, true),
            }
        })
        .0
}

pub fn tests() {
    let example1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let example2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part1(example1), 161);
    assert_eq!(part2(example2), 48);
}
