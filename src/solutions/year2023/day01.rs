fn part_(digits: &[(&str, u32)], input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let parse_digit = |i| {
                for &(k, v) in digits {
                    if i + k.len() <= line.len() && &line[i..i + k.len()] == k {
                        return Some(v);
                    }
                }
                None
            };
            let first = (0..line.len()).find_map(parse_digit).unwrap();
            let last = (0..line.len()).rev().find_map(parse_digit).unwrap();
            10 * first + last
        })
        .sum()
}

pub fn part1(input: &str) -> u32 {
    part_(
        &[
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ],
        input,
    )
}

pub fn part2(input: &str) -> u32 {
    part_(
        &[
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ],
        input,
    )
}

pub fn tests() {
    assert_eq!(part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
    assert_eq!(
        part2(
            &[
                "two1nine",
                "eightwothree",
                "abcone2threexyz",
                "xtwone3four",
                "4nineeightseven2",
                "zoneight234",
                "7pqrstsixteen",
            ]
            .join("\n"),
        ),
        281,
    );
}
