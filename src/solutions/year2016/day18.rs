fn part_(rows: u32, input: &str) -> usize {
    // We can represent a row of tiles as a single u128
    let mut tiles = 0u128;
    for (i, c) in input.chars().enumerate() {
        if c == '^' {
            tiles |= 1 << i;
        }
    }

    let mut res = input.len() - tiles.count_ones() as usize;
    for _ in 1..rows {
        // The given rules simplify to an XOR (the center tile is irrelevant)
        tiles = ((tiles << 1) ^ (tiles >> 1)) % (1 << input.len());
        res += input.len() - tiles.count_ones() as usize;
    }
    res
}

pub fn part1(input: &str) -> usize {
    part_(40, input)
}

pub fn part2(input: &str) -> usize {
    part_(400_000, input)
}

pub fn tests() {
    assert_eq!(part_(3, "..^^."), 6);
    assert_eq!(part_(10, ".^^.^.^^^^"), 38);
}
