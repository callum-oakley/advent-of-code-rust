use std::{collections::HashMap, iter};

// https://youtu.be/etMJxB-igrc
fn van_eck(mut init: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    let mut seen = HashMap::new();
    let mut i = 0;
    let mut next = init.next().unwrap();
    iter::from_fn(move || {
        let current = next;
        next = if let Some(x) = init.next() {
            x
        } else if let Some(j) = seen.get(&current) {
            i - j
        } else {
            0
        };
        seen.insert(current, i);
        i += 1;
        Some(current)
    })
}

fn part_(input: &str, n: usize) -> i32 {
    van_eck(input.split(',').map(|n| n.parse().unwrap()))
        .nth(n - 1)
        .unwrap()
}

pub fn part1(input: &str) -> i32 {
    part_(input, 2020)
}

pub fn part2(input: &str) -> i32 {
    part_(input, 30_000_000)
}

pub fn tests() {
    assert_eq!(part1("0,3,6"), 436);
    assert_eq!(part2("0,3,6"), 175_594);
}
