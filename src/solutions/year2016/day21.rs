enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBased(u8),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
    InvertRotateBased(u8),
}

impl Instruction {
    fn invert(self) -> Self {
        match self {
            Instruction::RotateLeft(x) => Instruction::RotateRight(x),
            Instruction::RotateRight(x) => Instruction::RotateLeft(x),
            Instruction::RotateBased(x) => Instruction::InvertRotateBased(x),
            Instruction::MovePosition(x, y) => Instruction::MovePosition(y, x),
            _ => self,
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            match (words.next().unwrap(), words.next().unwrap()) {
                ("swap", "position") => Instruction::SwapPosition(
                    words.next().unwrap().parse().unwrap(),
                    words.nth(2).unwrap().parse().unwrap(),
                ),
                ("swap", "letter") => Instruction::SwapLetter(
                    words.next().unwrap().as_bytes()[0],
                    words.nth(2).unwrap().as_bytes()[0],
                ),
                ("rotate", "left") => {
                    Instruction::RotateLeft(words.next().unwrap().parse().unwrap())
                }
                ("rotate", "right") => {
                    Instruction::RotateRight(words.next().unwrap().parse().unwrap())
                }
                ("rotate", "based") => {
                    Instruction::RotateBased(words.nth(4).unwrap().as_bytes()[0])
                }
                ("reverse", "positions") => Instruction::ReversePositions(
                    words.next().unwrap().parse().unwrap(),
                    words.nth(1).unwrap().parse().unwrap(),
                ),
                ("move", "position") => Instruction::MovePosition(
                    words.next().unwrap().parse().unwrap(),
                    words.nth(2).unwrap().parse().unwrap(),
                ),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn rotate_based(password: &mut [u8], x: u8) {
    let i = password.iter().position(|c| *c == x).unwrap();
    let j = (i + if i >= 4 { 2 } else { 1 }) % password.len();
    password.rotate_right(j);
}

fn scramble(password: &str, instructions: impl Iterator<Item = Instruction>) -> String {
    let mut password = Vec::from(password);
    for instruction in instructions {
        match instruction {
            Instruction::SwapPosition(x, y) => password.swap(x, y),
            Instruction::SwapLetter(x, y) => {
                let i = password.iter().position(|c| *c == x).unwrap();
                let j = password.iter().position(|c| *c == y).unwrap();
                password.swap(i, j);
            }
            Instruction::RotateLeft(x) => password.rotate_left(x),
            Instruction::RotateRight(x) => password.rotate_right(x),
            Instruction::RotateBased(x) => rotate_based(&mut password, x),
            Instruction::ReversePositions(x, y) => password[x..=y].reverse(),
            Instruction::MovePosition(x, y) => {
                let a = password.remove(x);
                password.insert(y, a);
            }
            Instruction::InvertRotateBased(x) => {
                let mut inverse = password.clone();
                loop {
                    let mut p = inverse.clone();
                    rotate_based(&mut p, x);
                    if p == password {
                        break;
                    }
                    inverse.rotate_left(1);
                }
                password = inverse;
            }
        }
    }
    String::from_utf8(password).unwrap()
}

fn part1_(password: &str, input: &str) -> String {
    scramble(password, parse(input).into_iter())
}

fn part2_(password: &str, input: &str) -> String {
    scramble(
        password,
        parse(input).into_iter().rev().map(Instruction::invert),
    )
}

pub fn part1(input: &str) -> String {
    part1_("abcdefgh", input)
}

pub fn part2(input: &str) -> String {
    part2_("fbgdceah", input)
}

pub fn tests() {
    let example = "swap position 4 with position 0
                   swap letter d with letter b
                   reverse positions 0 through 4
                   rotate left 1 step
                   move position 1 to position 4
                   move position 3 to position 0
                   rotate based on position of letter b
                   rotate based on position of letter d";
    assert_eq!(part1_("abcde", example), "decab");
    assert_eq!(part2_("decab", example), "abcde");
}
