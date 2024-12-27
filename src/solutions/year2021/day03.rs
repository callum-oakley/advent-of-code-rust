use crate::freqs::Freqs;

fn parse(input: &str) -> (usize, Vec<u32>) {
    (
        input.split_whitespace().next().unwrap().len(),
        input
            .split_whitespace()
            .map(|s| u32::from_str_radix(s, 2).unwrap())
            .collect(),
    )
}

fn most_common_bit(report: &[u32], i: usize) -> u32 {
    let freqs = report.iter().map(|&n| (n >> i) & 1).freqs();
    [0, 1].into_iter().max_by_key(|bit| freqs[bit]).unwrap()
}

fn rating(input: &str, mut target_bit: impl FnMut(&[u32], usize) -> u32) -> u32 {
    let (width, mut report) = parse(input);
    let mut i = width - 1;
    while report.len() > 1 {
        let bit = target_bit(&report, i);
        report.retain(|n| (n >> i) & 1 == bit);
        i -= 1;
    }
    report[0]
}

pub fn part1(input: &str) -> u32 {
    let (width, report) = parse(input);
    let (gamma, epsilon) = (0..width)
        .rev()
        .map(|i| {
            let bit = most_common_bit(&report, i);
            (bit, bit ^ 1)
        })
        .fold((0, 0), |(gamma, epsilon), (gbit, ebit)| {
            ((gamma << 1) | gbit, (epsilon << 1) | ebit)
        });
    gamma * epsilon
}

pub fn part2(input: &str) -> u32 {
    rating(input, most_common_bit) * rating(input, |report, i| most_common_bit(report, i) ^ 1)
}

pub fn tests() {
    let example = "00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010";
    assert_eq!(part1(example), 198);
    assert_eq!(part2(example), 230);
}
