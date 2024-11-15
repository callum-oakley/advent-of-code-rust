fn parse(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// false positive: https://github.com/rust-lang/rust-clippy/issues/13185
#[expect(clippy::manual_inspect)]
pub fn part1(input: &str) -> usize {
    parse(input)
        .chunks_mut(3)
        .map(|triangle| {
            triangle.sort_unstable();
            triangle
        })
        .filter(|triangle| triangle[0] + triangle[1] > triangle[2])
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .chunks(9)
        .flat_map(|chunk| {
            [
                [chunk[0], chunk[3], chunk[6]],
                [chunk[1], chunk[4], chunk[7]],
                [chunk[2], chunk[5], chunk[8]],
            ]
        })
        .map(|mut triangle| {
            triangle.sort_unstable();
            triangle
        })
        .filter(|triangle| triangle[0] + triangle[1] > triangle[2])
        .count()
}
