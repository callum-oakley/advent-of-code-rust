use regex::Regex;

use crate::number_theory;

fn parse(input: &str) -> Vec<number_theory::Congruence> {
    Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)\.")
        .unwrap()
        .captures_iter(input)
        .map(|captures| {
            let disc: i64 = captures[1].parse().unwrap();
            let positions: i64 = captures[2].parse().unwrap();
            let start: i64 = captures[3].parse().unwrap();
            // Want time so that start + disc + time = 0 (mod positions)
            // => time = 0 - disc - start (mod positions)
            number_theory::Congruence {
                a: 0 - disc - start,
                n: positions,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    number_theory::chinese_remainder(parse(input))
}

pub fn part2(input: &str) -> i64 {
    let mut system = parse(input);
    system.push(number_theory::Congruence { a: -7, n: 11 });
    number_theory::chinese_remainder(system)
}

pub fn tests() {
    assert_eq!(
        part1(
            "Disc #1 has 5 positions; at time=0, it is at position 4.
             Disc #2 has 2 positions; at time=0, it is at position 1.",
        ),
        5,
    );
}
