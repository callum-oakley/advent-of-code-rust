use std::collections::HashSet;

fn parse(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|l| {
            let (op, n) = l.split_once(' ').unwrap();
            (op, n.parse().unwrap())
        })
        .collect()
}

fn run(instructions: &[(&str, i32)]) -> Result<i32, i32> {
    let mut seen = HashSet::new();
    let mut ip = 0;
    let mut acc = 0;
    while !seen.contains(&ip) {
        seen.insert(ip);
        if usize::try_from(ip).unwrap() == instructions.len() {
            return Ok(acc);
        }
        match instructions[usize::try_from(ip).unwrap()] {
            ("acc", n) => {
                acc += n;
                ip += 1;
            }
            ("jmp", n) => {
                ip += n;
            }
            ("nop", _) => {
                ip += 1;
            }
            (op, _) => panic!("unsupported operation: {op}"),
        }
    }
    Err(acc)
}

fn modified(instructions: Vec<(&str, i32)>) -> impl Iterator<Item = Vec<(&str, i32)>> {
    (0..instructions.len()).filter_map(move |i| match instructions[i].0 {
        "jmp" => {
            let mut instructions = instructions.clone();
            instructions[i].0 = "nop";
            Some(instructions)
        }
        "nop" => {
            let mut instructions = instructions.clone();
            instructions[i].0 = "jmp";
            Some(instructions)
        }
        _ => None,
    })
}

pub fn part1(input: &str) -> i32 {
    run(&parse(input)).unwrap_err()
}

pub fn part2(input: &str) -> i32 {
    for instructions in modified(parse(input)) {
        if let Ok(acc) = run(&instructions) {
            return acc;
        }
    }
    unreachable!()
}

pub fn tests() {
    let example = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
    assert_eq!(part1(example), 5);
    assert_eq!(part2(example), 8);
}
