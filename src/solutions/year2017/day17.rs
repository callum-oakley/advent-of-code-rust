pub fn part1(input: &str) -> u32 {
    let step: usize = input.parse().unwrap();

    let mut buf = vec![0];
    let mut i = 0;
    for j in 1..=2017 {
        i = (i + step) % buf.len();
        buf.insert(i + 1, j);
        i += 1;
    }

    buf[(i + 1) % buf.len()]
}

/// Modify part1 to keep track only of the length of the buffer and the value at
/// index 1.
pub fn part2(input: &str) -> u32 {
    let step: usize = input.parse().unwrap();

    let mut buf_len = 1;
    let mut buf_1 = 0;
    let mut i = 0;
    for j in 1..=50_000_000 {
        i = (i + step) % buf_len;
        buf_len += 1;
        i += 1;
        if i == 1 {
            buf_1 = j;
        }
    }

    buf_1
}

pub fn tests() {
    assert_eq!(part1("3"), 638);
}
