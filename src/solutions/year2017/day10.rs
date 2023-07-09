use crate::hash;

pub fn part1(input: &str) -> u32 {
    let hash = hash::sparse(
        256,
        &input
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>(),
        1,
    );
    u32::from(hash[0]) * u32::from(hash[1])
}

pub fn part2(input: &str) -> String {
    format!("{:x}", hash::knot(input))
}

pub fn tests() {
    assert_eq!(hash::sparse(5, &[3, 4, 1, 5], 1), vec![3, 4, 2, 1, 0]);
    assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}
