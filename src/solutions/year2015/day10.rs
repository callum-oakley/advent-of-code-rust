use std::fmt::Write;

// TODO it would be nice to rewrite this in terms of group_by when that hits
// stable: https://doc.rust-lang.org/std/primitive.slice.html#method.group_by
fn look_and_say(s: &str) -> String {
    let mut res = String::new();
    let mut digit = s.chars().next().unwrap();
    let mut count = 1;
    for c in s.chars().skip(1) {
        if c == digit {
            count += 1;
        } else {
            write!(res, "{count}{digit}").unwrap();
            digit = c;
            count = 1;
        }
    }
    write!(res, "{count}{digit}").unwrap();
    res
}

fn part_(n: usize, input: &str) -> usize {
    let mut res = input.to_owned();
    for _ in 0..n {
        res = look_and_say(&res);
    }
    res.len()
}

pub fn part1(input: &str) -> usize {
    part_(40, input)
}

pub fn part2(input: &str) -> usize {
    part_(50, input)
}

pub fn tests() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");
}
