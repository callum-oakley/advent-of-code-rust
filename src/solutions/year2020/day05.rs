fn parse(s: &str) -> u32 {
    u32::from_str_radix(&s.replace(['F', 'L'], "0").replace(['B', 'R'], "1"), 2).unwrap()
}

pub fn part1(input: &str) -> u32 {
    input.lines().map(parse).max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let mut ids: Vec<u32> = input.lines().map(parse).collect();
    ids.sort_unstable();
    for pair in ids.windows(2) {
        if pair[1] - pair[0] > 1 {
            return pair[0] + 1;
        }
    }
    unreachable!()
}

pub fn tests() {
    assert_eq!(parse("FBFBBFFRLR"), 357);
    assert_eq!(parse("BFFFBBFRRR"), 567);
    assert_eq!(parse("FFFBBBFRRR"), 119);
    assert_eq!(parse("BBFFBBFRLL"), 820);
}
