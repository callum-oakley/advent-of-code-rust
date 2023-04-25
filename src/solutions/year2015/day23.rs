use std::collections::BTreeMap;

use regex::Regex;

type Register = char;

enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new("[, ]+").unwrap();
    input
        .lines()
        .map(|line| {
            let mut words = re.split(line);
            match words.next().unwrap() {
                "hlf" => Instruction::Hlf(words.next().unwrap().chars().next().unwrap()),
                "tpl" => Instruction::Tpl(words.next().unwrap().chars().next().unwrap()),
                "inc" => Instruction::Inc(words.next().unwrap().chars().next().unwrap()),
                "jmp" => Instruction::Jmp(words.next().unwrap().parse().unwrap()),
                "jie" => Instruction::Jie(
                    words.next().unwrap().chars().next().unwrap(),
                    words.next().unwrap().parse().unwrap(),
                ),
                "jio" => Instruction::Jio(
                    words.next().unwrap().chars().next().unwrap(),
                    words.next().unwrap().parse().unwrap(),
                ),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn run(a: i32, instructions: &[Instruction]) -> i32 {
    let mut registers: BTreeMap<_, _> = [('a', a), ('b', 0)].into();
    let mut ip = 0;
    while 0 <= ip && usize::try_from(ip).unwrap() < instructions.len() {
        match &instructions[usize::try_from(ip).unwrap()] {
            Instruction::Hlf(r) => *registers.entry(*r).or_insert(0) /= 2,
            Instruction::Tpl(r) => *registers.entry(*r).or_insert(0) *= 3,
            Instruction::Inc(r) => *registers.entry(*r).or_insert(0) += 1,
            Instruction::Jmp(offset) => {
                ip += offset;
                continue;
            }
            Instruction::Jie(r, offset) => {
                if registers[r] % 2 == 0 {
                    ip += offset;
                    continue;
                }
            }
            Instruction::Jio(r, offset) => {
                if registers[r] == 1 {
                    ip += offset;
                    continue;
                }
            }
        }
        ip += 1;
    }
    registers[&'b']
}

pub fn part1(input: &str) -> i32 {
    run(0, &parse(input))
}

pub fn part2(input: &str) -> i32 {
    run(1, &parse(input))
}
