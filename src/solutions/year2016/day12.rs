use std::collections::HashMap;

type Reg = char;

#[derive(PartialEq, Eq, Hash)]
enum Arg {
    Lit(i32),
    Reg(Reg),
}

impl Arg {
    fn value(&self, regs: &HashMap<Reg, i32>) -> i32 {
        match self {
            Arg::Lit(lit) => *lit,
            Arg::Reg(reg) => regs[reg],
        }
    }
}

impl From<&str> for Arg {
    fn from(s: &str) -> Self {
        if let Ok(n) = s.parse() {
            Arg::Lit(n)
        } else {
            Arg::Reg(s.chars().next().unwrap())
        }
    }
}

enum Instruction {
    Cpy(Arg, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Arg, Arg),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let op = words.next();
            let x = words.next();
            let y = words.next();
            match op.unwrap() {
                "cpy" => Instruction::Cpy(x.unwrap().into(), y.unwrap().chars().next().unwrap()),
                "inc" => Instruction::Inc(x.unwrap().chars().next().unwrap()),
                "dec" => Instruction::Dec(x.unwrap().chars().next().unwrap()),
                "jnz" => Instruction::Jnz(x.unwrap().into(), y.unwrap().into()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_(c: i32, input: &str) -> i32 {
    let instructions = parse(input);

    let mut regs = HashMap::new();
    regs.insert('c', c);

    let mut ip = 0;
    while ip < instructions.len() {
        match &instructions[ip] {
            Instruction::Cpy(x, y) => {
                regs.insert(*y, x.value(&regs));
            }
            Instruction::Inc(x) => {
                *regs.entry(*x).or_insert(0) += 1;
            }
            Instruction::Dec(x) => {
                *regs.entry(*x).or_insert(0) -= 1;
            }
            Instruction::Jnz(x, y) => {
                if x.value(&regs) != 0 {
                    let y = y.value(&regs);
                    if y < 0 {
                        ip -= usize::try_from(-y).unwrap();
                    } else {
                        ip += usize::try_from(y).unwrap();
                    }
                    continue;
                }
            }
        }
        ip += 1;
    }
    regs[&'a']
}

pub fn part1(input: &str) -> i32 {
    part_(0, input)
}

pub fn part2(input: &str) -> i32 {
    part_(1, input)
}

pub fn tests() {
    let example = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a";
    assert_eq!(part1(example), 42);
}
