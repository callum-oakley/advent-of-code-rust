fn parse(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    })
}

fn extrapolate(seq: &[i32]) -> i32 {
    if seq.iter().all(|&n| n == 0) {
        0
    } else {
        *seq.last().unwrap()
            + extrapolate(
                &seq.windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<_>>(),
            )
    }
}

pub fn part1(input: &str) -> i32 {
    parse(input).map(|seq| extrapolate(&seq)).sum()
}

pub fn part2(input: &str) -> i32 {
    parse(input)
        .map(|mut seq| {
            seq.reverse();
            extrapolate(&seq)
        })
        .sum()
}

pub fn tests() {
    let example = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
    assert_eq!(part1(example), 114);
    assert_eq!(part2(example), 2);
}
