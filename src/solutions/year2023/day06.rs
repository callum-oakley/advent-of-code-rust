use std::iter;

use crate::search;

fn parse1(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let (times, distances) = input.split_once('\n').unwrap();
    iter::zip(
        times
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap()),
        distances
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap()),
    )
}

fn parse2(input: &str) -> (usize, usize) {
    let (time, distance) = input.split_once('\n').unwrap();
    (
        time.strip_prefix("Time:")
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap(),
        distance
            .strip_prefix("Distance:")
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap(),
    )
}

// The distance travelled is symmetric about time / 2, and is increasing in the first half, so we
// can binary search to find the first win, and then use the symmetry to find the whole range.
fn count_wins((time, distance): (usize, usize)) -> usize {
    let first_win = search::binary(0, time / 2, |i| (time - i) * i > distance);
    time - first_win * 2 + 1
}

pub fn part1(input: &str) -> usize {
    parse1(input).map(count_wins).product()
}

pub fn part2(input: &str) -> usize {
    count_wins(parse2(input))
}

pub fn tests() {
    let example = "Time:      7  15   30\nDistance:  9  40  200";
    assert_eq!(part1(example), 288);
    assert_eq!(part2(example), 71503);
}
