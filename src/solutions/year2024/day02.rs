fn parse(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.trim().lines().map(|report| {
        report
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect()
    })
}

fn safe(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|pair| (1..=3).contains(&(pair[1] - pair[0])))
        || report
            .windows(2)
            .all(|pair| (1..=3).contains(&(pair[0] - pair[1])))
}

pub fn part1(input: &str) -> usize {
    parse(input).filter(|report| safe(report)).count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .filter(|report| {
            safe(report)
                || (0..report.len()).any(|i| {
                    let mut report = report.clone();
                    report.remove(i);
                    safe(&report)
                })
        })
        .count()
}

pub fn tests() {
    let example = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";
    assert_eq!(part1(example), 2);
    assert_eq!(part2(example), 4);
}
