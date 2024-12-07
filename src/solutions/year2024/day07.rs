use std::ops::{Add, Mul};

fn parse(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> + '_ {
    input.trim().lines().map(|line| {
        let (lhs, rhs) = line.split_once(':').unwrap();
        (
            lhs.trim().parse().unwrap(),
            rhs.split_whitespace().map(|s| s.parse().unwrap()).collect(),
        )
    })
}

fn part_(ops: &[&dyn Fn(u64, u64) -> u64], input: &str) -> u64 {
    fn possible(ops: &[&dyn Fn(u64, u64) -> u64], target: u64, acc: u64, rest: &[u64]) -> bool {
        if rest.is_empty() {
            acc == target
        } else {
            ops.iter()
                .any(|op| possible(ops, target, op(acc, rest[0]), &rest[1..]))
        }
    }
    parse(input)
        .filter(|(lhs, rhs)| possible(ops, *lhs, rhs[0], &rhs[1..]))
        .map(|(lhs, _)| lhs)
        .sum()
}

pub fn part1(input: &str) -> u64 {
    part_(&[&Add::add, &Mul::mul], input)
}

pub fn part2(input: &str) -> u64 {
    fn cat(a: u64, b: u64) -> u64 {
        a * 10u64.pow(b.ilog10() + 1) + b
    }
    part_(&[&Add::add, &Mul::mul, &cat], input)
}

pub fn tests() {
    let example = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    ";
    assert_eq!(part1(example), 3749);
    assert_eq!(part2(example), 11387);
}
