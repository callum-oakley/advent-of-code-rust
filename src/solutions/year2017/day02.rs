fn parse(input: &str) -> impl Iterator<Item = Vec<u32>> + '_ {
    input
        .lines()
        .map(|row| row.split_whitespace().map(|v| v.parse().unwrap()).collect())
}

pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|row| {
            for (i, x) in row.iter().enumerate() {
                for y in &row[i + 1..] {
                    if x % y == 0 {
                        return x / y;
                    }
                    if y % x == 0 {
                        return y / x;
                    }
                }
            }
            unreachable!();
        })
        .sum()
}

pub fn tests() {
    assert_eq!(part1("5 1 9 5\n7 5 3\n2 4 6 8"), 18);
    assert_eq!(part2("5 9 2 8\n9 4 7 3\n3 8 6 5"), 9);
}
