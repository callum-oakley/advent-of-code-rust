fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32>> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|s| s.parse::<u32>().unwrap()))
}

pub fn part1(input: &str) -> u32 {
    parse(input).map(Iterator::sum).max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let mut calories: Vec<u32> = parse(input).map(Iterator::sum).collect();
    calories.sort_unstable();
    calories.iter().rev().take(3).sum()
}

pub fn tests() {
    let example = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    assert_eq!(part1(example), 24000);
    assert_eq!(part2(example), 45000);
}
