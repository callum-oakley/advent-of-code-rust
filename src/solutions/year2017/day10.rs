use std::{fmt::Write, ops::BitXor};

fn sparse_hash(n: usize, lens: &[u8], rounds: usize) -> Vec<u8> {
    let mut ring = (0..=u8::try_from(n - 1).unwrap()).collect::<Vec<_>>();
    let mut pos = 0;
    for (skip, i) in (0..lens.len() * rounds).enumerate() {
        let len = usize::from(lens[i % lens.len()]);
        for j in 0..len / 2 {
            ring.swap((pos + j) % n, (pos + len - 1 - j) % n);
        }
        pos += len + skip;
    }
    ring
}

pub fn part1(input: &str) -> u32 {
    let hash = sparse_hash(
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
    let mut lens = Vec::from(input);
    lens.extend_from_slice(&[17, 31, 73, 47, 23]);

    let hash = sparse_hash(256, &lens, 64);

    let mut res = String::new();
    for block in hash.chunks(16) {
        write!(&mut res, "{:>02x}", block.iter().fold(0, u8::bitxor)).unwrap();
    }
    res
}

pub fn tests() {
    assert_eq!(sparse_hash(5, &[3, 4, 1, 5], 1), vec![3, 4, 2, 1, 0]);
    assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}
