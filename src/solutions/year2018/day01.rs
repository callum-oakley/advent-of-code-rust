use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let mut freq = 0;
    let mut seen = HashSet::from([freq]);
    for change in input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .cycle()
    {
        freq += change;
        if seen.contains(&freq) {
            break;
        }
        seen.insert(freq);
    }
    freq
}

pub fn tests() {
    assert_eq!(part1("+1 -2 +3 +1"), 3);
    assert_eq!(part1("+1 +1 +1"), 3);
    assert_eq!(part1("+1 +1 -2"), 0);
    assert_eq!(part1("-1 -2 -3"), -6);

    assert_eq!(part2("+1 -2 +3 +1"), 2);
    assert_eq!(part2("+1 -1"), 0);
    assert_eq!(part2("+3 +3 +4 -2 -4"), 10);
    assert_eq!(part2("-6 +3 +8 +5 -6"), 5);
    assert_eq!(part2("+7 +7 -2 -7 -4"), 14);
}
