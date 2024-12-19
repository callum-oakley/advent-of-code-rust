fn parse(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

pub fn tests() {
    let example = "199 200 208 210 200 207 240 269 260 263";
    assert_eq!(part1(example), 7);
    assert_eq!(part2(example), 5);
}
