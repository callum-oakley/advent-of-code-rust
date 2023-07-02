fn part_(offset: usize, input: &str) -> u32 {
    let input = input.as_bytes();
    let mut res: u32 = 0;
    for i in 0..input.len() {
        if input[i] == input[(i + offset) % input.len()] {
            res += u32::from(input[i] - b'0');
        }
    }
    res
}

pub fn part1(input: &str) -> u32 {
    part_(1, input)
}

pub fn part2(input: &str) -> u32 {
    part_(input.len() / 2, input)
}

pub fn tests() {
    assert_eq!(part1("1122"), 3);
    assert_eq!(part1("1111"), 4);
    assert_eq!(part1("1234"), 0);
    assert_eq!(part1("91212129"), 9);

    assert_eq!(part2("1212"), 6);
    assert_eq!(part2("1221"), 0);
    assert_eq!(part2("123425"), 4);
    assert_eq!(part2("123123"), 12);
    assert_eq!(part2("12131415"), 4);
}
