use std::ops::BitXor;

pub fn sparse(n: usize, lens: &[u8], rounds: usize) -> Vec<u8> {
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

pub fn knot(input: &str) -> u128 {
    let mut lens = Vec::from(input);
    lens.extend_from_slice(&[17, 31, 73, 47, 23]);

    let hash = sparse(256, &lens, 64);

    let mut res = 0;
    for (i, block) in hash.chunks(16).enumerate() {
        res |= u128::from(block.iter().fold(0, u8::bitxor)) << (120 - 8 * i);
    }
    res
}
