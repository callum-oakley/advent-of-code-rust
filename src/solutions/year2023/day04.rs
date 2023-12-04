fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    fn parse_nums(s: &str) -> impl Iterator<Item = usize> + '_ {
        s.split_whitespace().map(|n| n.parse().unwrap())
    }
    input.lines().map(|line| {
        let (left, right) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        // We're only interested in the number of wins, so calculate that here.
        let winning_numbers = parse_nums(left).collect::<Vec<_>>();
        parse_nums(right)
            .filter(|n| winning_numbers.contains(n))
            .count()
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|card| if card == 0 { 0 } else { 1 << (card - 1) })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let cards = parse(input).collect::<Vec<_>>();
    let mut copies = vec![1; cards.len()];
    let mut res = 0;
    for (i, card) in cards.into_iter().enumerate() {
        res += copies[i];
        for j in 1..=card {
            copies[i + j] += copies[i];
        }
    }
    res
}

pub fn tests() {
    let example = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ]
    .join("\n");

    assert_eq!(part1(&example), 13);
    assert_eq!(part2(&example), 30);
}
