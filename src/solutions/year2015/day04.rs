use md5::{Digest, Md5};

fn starts_with_zeroes(zeroes: usize, hash: &[u8]) -> bool {
    for i in 0..zeroes {
        let nibble = if i % 2 == 0 { 0xf0 } else { 0x0f };
        if hash[i / 2] & nibble != 0 {
            return false;
        }
    }
    true
}

fn part_(zeroes: usize, input: &str) -> usize {
    let mut hasher = Md5::new();
    let mut i = 0;
    loop {
        hasher.update(format!("{}{}", input, i));
        let hash = hasher.finalize_reset();
        if starts_with_zeroes(zeroes, &hash) {
            return i;
        }
        i += 1;
    }
}

pub fn part1(input: &str) -> usize {
    part_(5, input)
}

pub fn part2(input: &str) -> usize {
    part_(6, input)
}

pub fn tests() {
    assert_eq!(part1("abcdef"), 609043);
    assert_eq!(part1("pqrstuv"), 1048970);
}
