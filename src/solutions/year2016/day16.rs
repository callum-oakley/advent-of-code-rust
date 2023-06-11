fn parse(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '1').collect()
}

fn print(curve: Vec<bool>) -> String {
    curve
        .into_iter()
        .map(|bit| if bit { '1' } else { '0' })
        .collect()
}

fn dragon_fill(disk: usize, curve: &mut Vec<bool>) {
    while curve.len() < disk {
        let prev_len = curve.len();
        curve.push(false);
        for i in ((2 * prev_len + 1).saturating_sub(disk)..prev_len).rev() {
            curve.push(!curve[i]);
        }
    }
}

fn checksum(curve: &mut Vec<bool>) {
    while curve.len() % 2 == 0 {
        for i in (0..curve.len()).step_by(2) {
            curve[i / 2] = curve[i] == curve[i + 1];
        }
        curve.truncate(curve.len() / 2);
    }
}

fn part_(disk: usize, input: &str) -> String {
    let mut curve = parse(input);
    dragon_fill(disk, &mut curve);
    checksum(&mut curve);
    print(curve)
}

pub fn part1(input: &str) -> String {
    part_(272, input)
}

pub fn part2(input: &str) -> String {
    part_(35_651_584, input)
}

pub fn tests() {
    assert_eq!(part_(20, "10000"), "01100");
}
