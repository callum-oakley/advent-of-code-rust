pub fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut is_escape = false;
    for c in input.chars() {
        if is_escape {
            if c == 'x' {
                count += 3;
            } else {
                count += 1;
            }
            is_escape = false;
        } else {
            match c {
                '"' => count += 1,
                '\\' => is_escape = true,
                _ => (),
            }
        }
    }
    count
}

pub fn part2(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        count += 2;
        for c in line.chars() {
            if c == '"' || c == '\\' {
                count += 1;
            }
        }
    }
    count
}

pub fn tests() {
    assert_eq!(part1("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\""), 12);
    assert_eq!(part2("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\""), 19);
}
