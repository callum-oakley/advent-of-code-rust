fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        c.to_digit(36).unwrap() - 9
    } else {
        c.to_digit(36).unwrap() + 17
    }
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            priority(left.chars().find(|&item| right.contains(item)).unwrap())
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            priority(
                chunk[0]
                    .chars()
                    .find(|&item| chunk[1].contains(item) && chunk[2].contains(item))
                    .unwrap(),
            )
        })
        .sum()
}

pub fn tests() {
    let example = concat!(
        "vJrwpWtwJgWrhcsFMMfFFhFp\n",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
        "PmmdzqPrVvPwwTWBwg\n",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
        "ttgJtRGJQctTZtZT\n",
        "CrZsJsPPZsGzwwsLwLmpwMDw\n",
    );
    assert_eq!(part1(example), 157);
    assert_eq!(part2(example), 70);
}
